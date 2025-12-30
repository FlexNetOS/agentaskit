//! Location Manager
//!
//! Target location determination and file organization automation
//! following production structure preferences.

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::{DeliverableType, LocationType, TargetLocation};

/// Location Manager for target determination and organization
pub struct LocationManager {
    workspace_root: PathBuf,
    production_dir: PathBuf,
    location_mappings: HashMap<DeliverableType, LocationType>,
}

impl LocationManager {
    pub fn new(workspace_root: PathBuf) -> Self {
        let production_dir = workspace_root.join("agentaskit-production");
        
        let mut location_mappings = HashMap::new();
        location_mappings.insert(DeliverableType::SourceCode, LocationType::ProductionDirectory);
        location_mappings.insert(DeliverableType::Documentation, LocationType::DocsSubdirectory);
        location_mappings.insert(DeliverableType::Configuration, LocationType::ConfigDirectory);
        location_mappings.insert(DeliverableType::TestSuite, LocationType::TestDirectory);
        location_mappings.insert(DeliverableType::BuildArtifact, LocationType::ProductionDirectory);
        location_mappings.insert(DeliverableType::Deployment, LocationType::ScriptsDirectory);
        location_mappings.insert(DeliverableType::Report, LocationType::DocsSubdirectory);
        location_mappings.insert(DeliverableType::Analysis, LocationType::DocsSubdirectory);

        Self {
            workspace_root,
            production_dir,
            location_mappings,
        }
    }

    /// Resolve target location for a deliverable type
    pub fn resolve_location(&self, deliverable_type: &DeliverableType) -> Result<TargetLocation> {
        let location_type = self.location_mappings
            .get(deliverable_type)
            .cloned()
            .unwrap_or(LocationType::ProductionDirectory);

        let base_path = self.get_base_path(&location_type);
        let org_rules = self.get_organization_rules(&location_type);
        let backup_locations = self.get_backup_locations(&location_type);

        Ok(TargetLocation {
            location_type,
            base_path,
            relative_path: String::new(),
            filename_pattern: None,
            organization_rules: org_rules,
            backup_locations,
        })
    }

    /// Get base path for location type
    fn get_base_path(&self, location_type: &LocationType) -> PathBuf {
        match location_type {
            LocationType::ProductionDirectory => self.production_dir.clone(),
            LocationType::DocsSubdirectory => self.workspace_root.join("docs"),
            LocationType::TestDirectory => self.production_dir.join("tests"),
            LocationType::ConfigDirectory => self.production_dir.join("configs"),
            LocationType::ScriptsDirectory => self.production_dir.join("scripts"),
            LocationType::ArchiveDirectory => self.workspace_root.join("archive"),
            LocationType::TempDirectory => self.workspace_root.join("temp"),
        }
    }

    /// Get organization rules for location type
    fn get_organization_rules(&self, location_type: &LocationType) -> Vec<String> {
        match location_type {
            LocationType::ProductionDirectory => vec![
                "Primary production codebase must reside in agentaskit-production directory".to_string(),
                "Follow Rust workspace structure conventions".to_string(),
                "No loose files at production root".to_string(),
            ],
            LocationType::DocsSubdirectory => vec![
                "Only single-source-of-truth.md allowed at root level".to_string(),
                "All other artifacts must be organized in ~/docs subdirectory".to_string(),
                "Use descriptive filenames with timestamps for reports".to_string(),
            ],
            LocationType::TestDirectory => vec![
                "Integration tests in tests/ directory".to_string(),
                "Test files must mirror source structure".to_string(),
                "Use *_test.rs naming convention".to_string(),
            ],
            LocationType::ConfigDirectory => vec![
                "Separate production, staging, and development configs".to_string(),
                "Use environment-specific subdirectories".to_string(),
                "Sensitive config in secrets/ subdirectory".to_string(),
            ],
            LocationType::ScriptsDirectory => vec![
                "Executable scripts with proper permissions".to_string(),
                "Document usage and dependencies".to_string(),
                "Organize by function (build, deploy, maintenance)".to_string(),
            ],
            LocationType::ArchiveDirectory => vec![
                "Timestamped directory structure for versions".to_string(),
                "Include manifest for each archived version".to_string(),
                "Compress large archives".to_string(),
            ],
            LocationType::TempDirectory => vec![
                "Temporary files only - not version controlled".to_string(),
                "Clean up after processing".to_string(),
                "Include .gitignore for temp directory".to_string(),
            ],
        }
    }

    /// Get backup locations for location type
    fn get_backup_locations(&self, location_type: &LocationType) -> Vec<PathBuf> {
        let archive_base = self.workspace_root.join("archive");
        
        match location_type {
            LocationType::ProductionDirectory => vec![
                archive_base.join("production_backups"),
            ],
            LocationType::DocsSubdirectory => vec![
                archive_base.join("docs_backups"),
            ],
            LocationType::TestDirectory => vec![
                archive_base.join("test_backups"),
            ],
            LocationType::ConfigDirectory => vec![
                archive_base.join("config_backups"),
                self.production_dir.join("configs").join("backup"),
            ],
            LocationType::ScriptsDirectory => vec![
                archive_base.join("script_backups"),
            ],
            _ => vec![archive_base.join("misc_backups")],
        }
    }

    /// Resolve full path for a specific deliverable
    pub fn resolve_full_path(
        &self,
        deliverable_type: &DeliverableType,
        relative_path: &str,
        filename: &str,
    ) -> Result<PathBuf> {
        let location = self.resolve_location(deliverable_type)?;
        let mut full_path = location.base_path;
        
        if !relative_path.is_empty() {
            full_path = full_path.join(relative_path);
        }
        
        full_path = full_path.join(filename);
        
        Ok(full_path)
    }

    /// Validate path against organization rules
    pub fn validate_path(&self, path: &Path, location_type: &LocationType) -> Result<ValidationReport> {
        let mut issues = Vec::new();
        let rules = self.get_organization_rules(location_type);
        
        // Check if path is within expected base
        let base_path = self.get_base_path(location_type);
        if !path.starts_with(&base_path) {
            issues.push(format!(
                "Path {} is not within expected base {}",
                path.display(),
                base_path.display()
            ));
        }
        
        // Check for root-level violations (DocsSubdirectory)
        if matches!(location_type, LocationType::DocsSubdirectory) {
            if let Some(parent) = path.parent() {
                if parent == self.workspace_root && path.file_name().unwrap_or_default() != "single-source-of-truth.md" {
                    issues.push("Only single-source-of-truth.md allowed at root level".to_string());
                }
            }
        }
        
        Ok(ValidationReport {
            is_valid: issues.is_empty(),
            issues,
            rules_checked: rules,
        })
    }

    /// Create directory structure if it doesn't exist
    pub async fn ensure_directory_structure(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .context("Failed to create directory structure")?;
            }
        }
        Ok(())
    }

    /// Get organized structure for a category
    pub fn get_category_structure(&self, category: &str) -> PathBuf {
        match category.to_lowercase().as_str() {
            "workflow" => self.production_dir.join("core/src/workflows"),
            "agent" => self.production_dir.join("core/src/agents"),
            "orchestration" => self.production_dir.join("core/src/orchestration"),
            "monitoring" => self.production_dir.join("core/src/monitoring"),
            "security" => self.production_dir.join("core/src/security"),
            "ui" => self.production_dir.join("core/src/ui"),
            "integration_test" => self.production_dir.join("tests/integration"),
            "performance_test" => self.production_dir.join("tests/performance"),
            "security_test" => self.production_dir.join("tests/security"),
            "architecture_docs" => self.workspace_root.join("docs/architecture"),
            "api_docs" => self.workspace_root.join("docs/api"),
            "deployment_docs" => self.workspace_root.join("docs/deployment"),
            "report" => self.workspace_root.join("docs/reports"),
            _ => self.production_dir.clone(),
        }
    }
}

/// Validation report for path checking
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub rules_checked: Vec<String>,
}

/// Resolve deliverable location (backward compatibility)
pub fn resolve(deliverable: &str) -> String {
    if deliverable.contains("source") || deliverable.contains("code") {
        "agentaskit-production/core/src/".to_string()
    } else if deliverable.contains("workflow") {
        "agentaskit-production/core/src/workflows/".to_string()
    } else if deliverable.contains("doc") {
        "docs/".to_string()
    } else if deliverable.contains("test") {
        "agentaskit-production/tests/".to_string()
    } else if deliverable.contains("config") {
        "agentaskit-production/configs/".to_string()
    } else {
        "agentaskit-production/".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        assert_eq!(resolve("source code"), "agentaskit-production/core/src/");
        assert_eq!(resolve("workflow code"), "agentaskit-production/core/src/workflows/");
        assert_eq!(resolve("documentation"), "docs/");
        assert_eq!(resolve("test suite"), "agentaskit-production/tests/");
    }

    #[test]
    fn test_location_manager() {
        let manager = LocationManager::new(PathBuf::from("d:/test/workspace"));
        
        let location = manager.resolve_location(&DeliverableType::SourceCode).unwrap();
        assert_eq!(location.location_type, LocationType::ProductionDirectory);
        
        let location = manager.resolve_location(&DeliverableType::Documentation).unwrap();
        assert_eq!(location.location_type, LocationType::DocsSubdirectory);
    }

    #[test]
    fn test_get_category_structure() {
        let manager = LocationManager::new(PathBuf::from("d:/test/workspace"));
        
        let path = manager.get_category_structure("workflow");
        assert!(path.to_string_lossy().contains("workflows"));
        
        let path = manager.get_category_structure("agent");
        assert!(path.to_string_lossy().contains("agents"));
    }
}
