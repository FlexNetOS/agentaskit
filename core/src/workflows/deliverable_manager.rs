//! Deliverable Manager
//!
//! Automated deliverable specification generation, target location determination,
//! file organization, and backup integration.

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{
    Deliverable, DeliverableType, TargetLocation, LocationType,
    FileSpecification, RequestPriority,
};

/// Deliverable Manager for comprehensive deliverable handling
pub struct DeliverableManager {
    base_production_path: PathBuf,
    backup_enabled: bool,
    organization_rules: HashMap<LocationType, Vec<String>>,
}

impl DeliverableManager {
    pub fn new(base_production_path: PathBuf) -> Self {
        let mut organization_rules = HashMap::new();
        
        // Define organization rules for each location type
        organization_rules.insert(
            LocationType::ProductionDirectory,
            vec![
                "All source code must be in appropriate subdirectories".to_string(),
                "No loose files at production root".to_string(),
                "Follow Rust workspace structure".to_string(),
            ],
        );
        
        organization_rules.insert(
            LocationType::DocsSubdirectory,
            vec![
                "Only single-source-of-truth.md allowed at root".to_string(),
                "All other artifacts in docs/ subdirectory".to_string(),
                "Use descriptive filenames with dates".to_string(),
            ],
        );
        
        organization_rules.insert(
            LocationType::TestDirectory,
            vec![
                "Test files must mirror source structure".to_string(),
                "Integration tests in tests/ directory".to_string(),
                "Unit tests alongside source files".to_string(),
            ],
        );

        Self {
            base_production_path,
            backup_enabled: true,
            organization_rules,
        }
    }

    /// Plan deliverable locations based on specification
    pub fn plan(&self, spec: &DeliverableSpec) -> Result<PlannedDeliverable> {
        let target_location = self.determine_target_location(
            &spec.deliverable_type,
            &spec.priority,
            &spec.category,
        )?;

        let file_specs = self.generate_file_specifications(&spec.deliverable_type)?;
        
        let backup_locations = if self.backup_enabled {
            self.determine_backup_locations(&target_location)?
        } else {
            Vec::new()
        };

        Ok(PlannedDeliverable {
            id: Uuid::new_v4(),
            spec: spec.clone(),
            target_location,
            file_specifications: file_specs,
            backup_locations,
            created_at: Utc::now(),
        })
    }

    /// Determine target location based on deliverable type and priority
    fn determine_target_location(
        &self,
        deliverable_type: &DeliverableType,
        priority: &RequestPriority,
        category: &str,
    ) -> Result<TargetLocation> {
        let location_type = match deliverable_type {
            DeliverableType::SourceCode => LocationType::ProductionDirectory,
            DeliverableType::Documentation => LocationType::DocsSubdirectory,
            DeliverableType::Configuration => LocationType::ConfigDirectory,
            DeliverableType::TestSuite => LocationType::TestDirectory,
            DeliverableType::BuildArtifact => LocationType::ProductionDirectory,
            DeliverableType::Deployment => LocationType::ScriptsDirectory,
            DeliverableType::Report => LocationType::DocsSubdirectory,
            DeliverableType::Analysis => LocationType::DocsSubdirectory,
        };

        let base_path = self.get_base_path_for_location(&location_type);
        let relative_path = self.generate_relative_path(deliverable_type, category)?;
        let filename_pattern = self.generate_filename_pattern(deliverable_type)?;
        let org_rules = self.organization_rules
            .get(&location_type)
            .cloned()
            .unwrap_or_default();
        let backup_locs = self.get_backup_locations_for_type(&location_type)?;

        Ok(TargetLocation {
            location_type,
            base_path,
            relative_path,
            filename_pattern: Some(filename_pattern),
            organization_rules: org_rules,
            backup_locations: backup_locs,
        })
    }

    /// Get base path for location type
    fn get_base_path_for_location(&self, location_type: &LocationType) -> PathBuf {
        match location_type {
            LocationType::ProductionDirectory => self.base_production_path.clone(),
            LocationType::DocsSubdirectory => self.base_production_path.join("docs"),
            LocationType::TestDirectory => self.base_production_path.join("tests"),
            LocationType::ConfigDirectory => self.base_production_path.join("configs"),
            LocationType::ScriptsDirectory => self.base_production_path.join("scripts"),
            LocationType::ArchiveDirectory => {
                self.base_production_path.parent().unwrap_or(&self.base_production_path).join("archive")
            }
            LocationType::TempDirectory => {
                self.base_production_path.join("temp")
            }
        }
    }

    /// Generate relative path within base location
    fn generate_relative_path(
        &self,
        deliverable_type: &DeliverableType,
        category: &str,
    ) -> Result<String> {
        let path = match deliverable_type {
            DeliverableType::SourceCode => {
                if category.contains("workflow") {
                    "core/src/workflows".to_string()
                } else if category.contains("agent") {
                    "core/src/agents".to_string()
                } else if category.contains("orchestration") {
                    "core/src/orchestration".to_string()
                } else {
                    "core/src".to_string()
                }
            }
            DeliverableType::Documentation => {
                if category.contains("report") {
                    "reports".to_string()
                } else if category.contains("architecture") {
                    "architecture".to_string()
                } else {
                    "".to_string()
                }
            }
            DeliverableType::Configuration => {
                if category.contains("production") {
                    "production".to_string()
                } else if category.contains("development") {
                    "development".to_string()
                } else {
                    "".to_string()
                }
            }
            DeliverableType::TestSuite => {
                if category.contains("integration") {
                    "integration".to_string()
                } else if category.contains("performance") {
                    "performance".to_string()
                } else {
                    "".to_string()
                }
            }
            DeliverableType::BuildArtifact => "artifacts".to_string(),
            DeliverableType::Deployment => "deploy".to_string(),
            DeliverableType::Report => "reports".to_string(),
            DeliverableType::Analysis => "analysis".to_string(),
        };

        Ok(path)
    }

    /// Generate filename pattern for deliverable type
    fn generate_filename_pattern(&self, deliverable_type: &DeliverableType) -> Result<String> {
        let pattern = match deliverable_type {
            DeliverableType::SourceCode => "{name}.rs",
            DeliverableType::Documentation => "{name}.md",
            DeliverableType::Configuration => "{name}.{yaml,toml,json}",
            DeliverableType::TestSuite => "{name}_test.rs",
            DeliverableType::BuildArtifact => "{name}.{tar.gz,zip,bin}",
            DeliverableType::Deployment => "{name}.sh",
            DeliverableType::Report => "{name}_report_{timestamp}.md",
            DeliverableType::Analysis => "{name}_analysis_{timestamp}.json",
        };

        Ok(pattern.to_string())
    }

    /// Generate file specifications for deliverable type
    fn generate_file_specifications(
        &self,
        deliverable_type: &DeliverableType,
    ) -> Result<Vec<FileSpecification>> {
        let specs = match deliverable_type {
            DeliverableType::SourceCode => vec![FileSpecification {
                filename: "module.rs".to_string(),
                file_type: "rust".to_string(),
                size_limits: Some((0, 1024 * 1024)), // 0-1MB
                format_requirements: vec![
                    "Valid Rust syntax".to_string(),
                    "Formatted with rustfmt".to_string(),
                    "Passes clippy lints".to_string(),
                ],
                encoding: "utf-8".to_string(),
                permissions: Some("644".to_string()),
            }],
            DeliverableType::Documentation => vec![FileSpecification {
                filename: "document.md".to_string(),
                file_type: "markdown".to_string(),
                size_limits: Some((0, 10 * 1024 * 1024)), // 0-10MB
                format_requirements: vec![
                    "Valid Markdown syntax".to_string(),
                    "Include metadata header".to_string(),
                ],
                encoding: "utf-8".to_string(),
                permissions: Some("644".to_string()),
            }],
            DeliverableType::Configuration => vec![FileSpecification {
                filename: "config.yaml".to_string(),
                file_type: "yaml".to_string(),
                size_limits: Some((0, 100 * 1024)), // 0-100KB
                format_requirements: vec![
                    "Valid YAML syntax".to_string(),
                    "Schema validated".to_string(),
                ],
                encoding: "utf-8".to_string(),
                permissions: Some("600".to_string()), // More restrictive for config
            }],
            _ => vec![],
        };

        Ok(specs)
    }

    /// Get backup locations for a location type
    fn get_backup_locations_for_type(&self, location_type: &LocationType) -> Result<Vec<PathBuf>> {
        if !self.backup_enabled {
            return Ok(Vec::new());
        }

        let archive_base = self.base_production_path
            .parent()
            .unwrap_or(&self.base_production_path)
            .join("archive");

        let backup_path = match location_type {
            LocationType::ProductionDirectory => archive_base.join("production_backups"),
            LocationType::DocsSubdirectory => archive_base.join("docs_backups"),
            LocationType::TestDirectory => archive_base.join("test_backups"),
            LocationType::ConfigDirectory => archive_base.join("config_backups"),
            LocationType::ScriptsDirectory => archive_base.join("script_backups"),
            _ => archive_base.join("misc_backups"),
        };

        Ok(vec![backup_path])
    }

    /// Determine backup locations for a specific target location
    fn determine_backup_locations(&self, target: &TargetLocation) -> Result<Vec<PathBuf>> {
        self.get_backup_locations_for_type(&target.location_type)
    }

    /// Validate deliverable against organization rules
    pub fn validate_deliverable(&self, deliverable: &PlannedDeliverable) -> Result<ValidationResult> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // Check file size limits
        for file_spec in &deliverable.file_specifications {
            if let Some((min, max)) = file_spec.size_limits {
                // Would need actual file size here - this is a placeholder
                if max < min {
                    violations.push(format!(
                        "Invalid size limits for {}: max < min",
                        file_spec.filename
                    ));
                }
            }
        }

        // Check organization rules compliance
        for rule in &deliverable.target_location.organization_rules {
            // Placeholder - would check actual compliance
            if rule.contains("required") && !rule.contains("optional") {
                // Could add actual rule checking here
            }
        }

        // Validate backup locations exist or can be created
        for backup_loc in &deliverable.backup_locations {
            if !backup_loc.exists() {
                warnings.push(format!(
                    "Backup location does not exist: {}",
                    backup_loc.display()
                ));
            }
        }

        Ok(ValidationResult {
            is_valid: violations.is_empty(),
            violations,
            warnings,
        })
    }

    /// Create backup of deliverable
    pub async fn create_backup(
        &self,
        deliverable: &PlannedDeliverable,
        source_path: &Path,
    ) -> Result<Vec<PathBuf>> {
        let mut created_backups = Vec::new();

        for backup_location in &deliverable.backup_locations {
            if !backup_location.exists() {
                tokio::fs::create_dir_all(&backup_location).await?;
            }

            let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
            let backup_filename = format!(
                "{}_{}.backup",
                source_path.file_name().unwrap_or_default().to_string_lossy(),
                timestamp
            );
            let backup_path = backup_location.join(backup_filename);

            tokio::fs::copy(source_path, &backup_path).await?;
            created_backups.push(backup_path);
        }

        Ok(created_backups)
    }
}

/// Deliverable specification for planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverableSpec {
    pub name: String,
    pub description: String,
    pub deliverable_type: DeliverableType,
    pub priority: RequestPriority,
    pub category: String,
}

/// Planned deliverable with all location and backup information
#[derive(Debug, Clone)]
pub struct PlannedDeliverable {
    pub id: Uuid,
    pub spec: DeliverableSpec,
    pub target_location: TargetLocation,
    pub file_specifications: Vec<FileSpecification>,
    pub backup_locations: Vec<PathBuf>,
    pub created_at: DateTime<Utc>,
}

/// Validation result for deliverable
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
}

/// Plan deliverable location (backward compatibility)
pub fn plan(spec: &str) -> String {
    // Simple planning function for backward compatibility
    if spec.contains("source") || spec.contains("code") {
        "agentaskit-production/core/src".to_string()
    } else if spec.contains("doc") || spec.contains("report") {
        "docs".to_string()
    } else if spec.contains("test") {
        "agentaskit-production/tests".to_string()
    } else {
        "agentaskit-production".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        let result = plan("source code");
        assert!(result.contains("core/src"));

        let result = plan("documentation");
        assert!(result.contains("docs"));
    }

    #[test]
    fn test_deliverable_manager() {
        let manager = DeliverableManager::new(PathBuf::from("agentaskit-production"));
        
        let spec = DeliverableSpec {
            name: "test_module".to_string(),
            description: "Test module".to_string(),
            deliverable_type: DeliverableType::SourceCode,
            priority: RequestPriority::High,
            category: "workflow".to_string(),
        };

        let planned = manager.plan(&spec).unwrap();
        assert_eq!(planned.target_location.location_type, LocationType::ProductionDirectory);
        assert!(planned.target_location.relative_path.contains("workflow"));
    }
}
