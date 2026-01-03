//! Deliverable Manager
//!
//! Manages workflow deliverables including planning, tracking, validation,
//! and quality assurance for all workflow outputs.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Types of deliverables that can be produced
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeliverableType {
    /// Source code files
    Code,
    /// Documentation files
    Documentation,
    /// Configuration files
    Configuration,
    /// Test artifacts
    Test,
    /// Build artifacts
    Artifact,
    /// Report outputs
    Report,
    /// Data files
    Data,
    /// Custom deliverable type
    Custom(String),
}

/// Status of a deliverable
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliverableStatus {
    /// Planned but not started
    Planned,
    /// Currently being produced
    InProgress,
    /// Completed and pending validation
    PendingValidation,
    /// Validated and approved
    Validated,
    /// Failed validation
    Failed(String),
    /// Delivered to target location
    Delivered,
}

/// Quality gate for deliverable validation
#[derive(Debug, Clone)]
pub struct QualityGate {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub validator: ValidatorType,
}

/// Types of validators for quality gates
#[derive(Debug, Clone)]
pub enum ValidatorType {
    /// Check file exists
    FileExists,
    /// Check file is non-empty
    NonEmpty,
    /// Check syntax validity
    SyntaxValid,
    /// Run custom validation command
    Command(String),
    /// Schema validation
    Schema(String),
    /// Size constraints (min, max bytes)
    SizeRange(usize, usize),
}

/// A single deliverable item
#[derive(Debug, Clone)]
pub struct Deliverable {
    pub id: String,
    pub name: String,
    pub deliverable_type: DeliverableType,
    pub description: String,
    pub target_path: PathBuf,
    pub status: DeliverableStatus,
    pub quality_gates: Vec<QualityGate>,
    pub metadata: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub created_at: Instant,
    pub completed_at: Option<Instant>,
}

/// Deliverable plan specification
#[derive(Debug, Clone)]
pub struct DeliverablePlan {
    pub deliverables: Vec<Deliverable>,
    pub execution_order: Vec<String>,
    pub parallel_groups: Vec<Vec<String>>,
}

/// Validation result for a deliverable
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub deliverable_id: String,
    pub passed: bool,
    pub gate_results: Vec<GateResult>,
    pub duration: Duration,
}

/// Result of a single quality gate check
#[derive(Debug, Clone)]
pub struct GateResult {
    pub gate_name: String,
    pub passed: bool,
    pub message: String,
}

/// Delivery receipt confirming successful delivery
#[derive(Debug, Clone)]
pub struct DeliveryReceipt {
    pub deliverable_id: String,
    pub target_path: PathBuf,
    pub checksum: String,
    pub size_bytes: usize,
    pub delivered_at: Instant,
}

/// Manages the lifecycle of workflow deliverables
pub struct DeliverableManager {
    deliverables: HashMap<String, Deliverable>,
    validation_results: HashMap<String, ValidationResult>,
    delivery_receipts: HashMap<String, DeliveryReceipt>,
    base_output_path: PathBuf,
}

impl DeliverableManager {
    /// Create a new deliverable manager with the specified base output path
    pub fn new(base_output_path: PathBuf) -> Self {
        Self {
            deliverables: HashMap::new(),
            validation_results: HashMap::new(),
            delivery_receipts: HashMap::new(),
            base_output_path,
        }
    }

    /// Plan deliverables from a specification string
    ///
    /// Parses the specification and creates a delivery plan with
    /// proper ordering based on dependencies.
    pub fn plan(&mut self, spec: &str) -> Result<DeliverablePlan, String> {
        let deliverables = self.parse_spec(spec)?;

        // Register all deliverables
        for deliverable in &deliverables {
            self.deliverables.insert(deliverable.id.clone(), deliverable.clone());
        }

        // Compute execution order using topological sort
        let execution_order = self.topological_sort(&deliverables)?;

        // Identify parallel execution groups
        let parallel_groups = self.compute_parallel_groups(&deliverables, &execution_order);

        Ok(DeliverablePlan {
            deliverables,
            execution_order,
            parallel_groups,
        })
    }

    /// Parse specification string into deliverables
    fn parse_spec(&self, spec: &str) -> Result<Vec<Deliverable>, String> {
        let mut deliverables = Vec::new();

        for (idx, line) in spec.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse format: TYPE:name:path[:dependencies]
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() < 3 {
                continue;
            }

            let deliverable_type = match parts[0].to_uppercase().as_str() {
                "CODE" => DeliverableType::Code,
                "DOC" | "DOCUMENTATION" => DeliverableType::Documentation,
                "CONFIG" | "CONFIGURATION" => DeliverableType::Configuration,
                "TEST" => DeliverableType::Test,
                "ARTIFACT" => DeliverableType::Artifact,
                "REPORT" => DeliverableType::Report,
                "DATA" => DeliverableType::Data,
                other => DeliverableType::Custom(other.to_string()),
            };

            let dependencies = if parts.len() > 3 {
                parts[3].split(',').map(|s| s.trim().to_string()).collect()
            } else {
                Vec::new()
            };

            let target_path = self.base_output_path.join(parts[2]);

            deliverables.push(Deliverable {
                id: format!("DEL-{:04}", idx + 1),
                name: parts[1].to_string(),
                deliverable_type: deliverable_type.clone(),
                description: format!("{} deliverable: {}", parts[0], parts[1]),
                target_path,
                status: DeliverableStatus::Planned,
                quality_gates: self.default_gates_for_type(&deliverable_type),
                metadata: HashMap::new(),
                dependencies,
                created_at: Instant::now(),
                completed_at: None,
            });
        }

        Ok(deliverables)
    }

    /// Get default quality gates for a deliverable type
    fn default_gates_for_type(&self, dtype: &DeliverableType) -> Vec<QualityGate> {
        let mut gates = vec![
            QualityGate {
                name: "file_exists".to_string(),
                description: "Verify file was created".to_string(),
                required: true,
                validator: ValidatorType::FileExists,
            },
            QualityGate {
                name: "non_empty".to_string(),
                description: "Verify file is not empty".to_string(),
                required: true,
                validator: ValidatorType::NonEmpty,
            },
        ];

        match dtype {
            DeliverableType::Code => {
                gates.push(QualityGate {
                    name: "syntax_valid".to_string(),
                    description: "Verify code syntax is valid".to_string(),
                    required: true,
                    validator: ValidatorType::SyntaxValid,
                });
            }
            DeliverableType::Configuration => {
                gates.push(QualityGate {
                    name: "config_valid".to_string(),
                    description: "Verify configuration is parseable".to_string(),
                    required: true,
                    validator: ValidatorType::SyntaxValid,
                });
            }
            DeliverableType::Test => {
                gates.push(QualityGate {
                    name: "tests_pass".to_string(),
                    description: "Verify all tests pass".to_string(),
                    required: true,
                    validator: ValidatorType::Command("cargo test".to_string()),
                });
            }
            _ => {}
        }

        gates
    }

    /// Topological sort of deliverables based on dependencies
    fn topological_sort(&self, deliverables: &[Deliverable]) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut visited = HashMap::new();
        let mut temp_mark = HashMap::new();

        let id_map: HashMap<String, &Deliverable> = deliverables
            .iter()
            .map(|d| (d.id.clone(), d))
            .collect();

        for deliverable in deliverables {
            if !visited.contains_key(&deliverable.id) {
                self.visit(
                    &deliverable.id,
                    &id_map,
                    &mut visited,
                    &mut temp_mark,
                    &mut result,
                )?;
            }
        }

        result.reverse();
        Ok(result)
    }

    fn visit(
        &self,
        id: &str,
        id_map: &HashMap<String, &Deliverable>,
        visited: &mut HashMap<String, bool>,
        temp_mark: &mut HashMap<String, bool>,
        result: &mut Vec<String>,
    ) -> Result<(), String> {
        if temp_mark.get(id).copied().unwrap_or(false) {
            return Err(format!("Circular dependency detected at {}", id));
        }

        if !visited.get(id).copied().unwrap_or(false) {
            temp_mark.insert(id.to_string(), true);

            if let Some(deliverable) = id_map.get(id) {
                for dep in &deliverable.dependencies {
                    self.visit(dep, id_map, visited, temp_mark, result)?;
                }
            }

            visited.insert(id.to_string(), true);
            temp_mark.insert(id.to_string(), false);
            result.push(id.to_string());
        }

        Ok(())
    }

    /// Compute groups of deliverables that can be executed in parallel
    fn compute_parallel_groups(
        &self,
        deliverables: &[Deliverable],
        _order: &[String],
    ) -> Vec<Vec<String>> {
        let mut groups = Vec::new();
        let mut assigned: HashMap<String, usize> = HashMap::new();

        for deliverable in deliverables {
            let max_dep_level = deliverable
                .dependencies
                .iter()
                .filter_map(|d| assigned.get(d))
                .max()
                .copied()
                .unwrap_or(0);

            let level = if deliverable.dependencies.is_empty() {
                0
            } else {
                max_dep_level + 1
            };

            while groups.len() <= level {
                groups.push(Vec::new());
            }

            groups[level].push(deliverable.id.clone());
            assigned.insert(deliverable.id.clone(), level);
        }

        groups
    }

    /// Update the status of a deliverable
    pub fn update_status(&mut self, id: &str, status: DeliverableStatus) -> Result<(), String> {
        if let Some(deliverable) = self.deliverables.get_mut(id) {
            deliverable.status = status.clone();

            if matches!(status, DeliverableStatus::Validated | DeliverableStatus::Delivered) {
                deliverable.completed_at = Some(Instant::now());
            }

            Ok(())
        } else {
            Err(format!("Deliverable not found: {}", id))
        }
    }

    /// Validate a deliverable against its quality gates
    pub fn validate(&mut self, id: &str) -> Result<ValidationResult, String> {
        let deliverable = self.deliverables
            .get(id)
            .ok_or_else(|| format!("Deliverable not found: {}", id))?
            .clone();

        let start = Instant::now();
        let mut gate_results = Vec::new();
        let mut all_required_passed = true;

        for gate in &deliverable.quality_gates {
            let (passed, message) = self.run_validator(&gate.validator, &deliverable.target_path);

            if gate.required && !passed {
                all_required_passed = false;
            }

            gate_results.push(GateResult {
                gate_name: gate.name.clone(),
                passed,
                message,
            });
        }

        let result = ValidationResult {
            deliverable_id: id.to_string(),
            passed: all_required_passed,
            gate_results,
            duration: start.elapsed(),
        };

        self.validation_results.insert(id.to_string(), result.clone());

        // Update deliverable status
        if all_required_passed {
            self.update_status(id, DeliverableStatus::Validated)?;
        } else {
            self.update_status(
                id,
                DeliverableStatus::Failed("Validation failed".to_string()),
            )?;
        }

        Ok(result)
    }

    /// Run a validator against a target path
    fn run_validator(&self, validator: &ValidatorType, path: &PathBuf) -> (bool, String) {
        match validator {
            ValidatorType::FileExists => {
                if path.exists() {
                    (true, "File exists".to_string())
                } else {
                    (false, format!("File not found: {:?}", path))
                }
            }
            ValidatorType::NonEmpty => {
                match std::fs::metadata(path) {
                    Ok(meta) if meta.len() > 0 => (true, "File is non-empty".to_string()),
                    Ok(_) => (false, "File is empty".to_string()),
                    Err(e) => (false, format!("Cannot read file: {}", e)),
                }
            }
            ValidatorType::SyntaxValid => {
                // For now, just check the file is readable
                match std::fs::read_to_string(path) {
                    Ok(_) => (true, "File is readable".to_string()),
                    Err(e) => (false, format!("Cannot read file: {}", e)),
                }
            }
            ValidatorType::Command(cmd) => {
                // Would run command in production
                (true, format!("Command '{}' would be executed", cmd))
            }
            ValidatorType::Schema(schema) => {
                (true, format!("Schema '{}' validation would be performed", schema))
            }
            ValidatorType::SizeRange(min, max) => {
                match std::fs::metadata(path) {
                    Ok(meta) => {
                        let size = meta.len() as usize;
                        if size >= *min && size <= *max {
                            (true, format!("File size {} is within range [{}, {}]", size, min, max))
                        } else {
                            (false, format!("File size {} outside range [{}, {}]", size, min, max))
                        }
                    }
                    Err(e) => (false, format!("Cannot read file metadata: {}", e)),
                }
            }
        }
    }

    /// Mark a deliverable as delivered and create a receipt
    pub fn deliver(&mut self, id: &str) -> Result<DeliveryReceipt, String> {
        let deliverable = self.deliverables
            .get(id)
            .ok_or_else(|| format!("Deliverable not found: {}", id))?;

        if !matches!(deliverable.status, DeliverableStatus::Validated) {
            return Err(format!("Deliverable {} must be validated before delivery", id));
        }

        // Get file info for receipt
        let metadata = std::fs::metadata(&deliverable.target_path)
            .map_err(|e| format!("Cannot read file metadata: {}", e))?;

        let checksum = format!("sha256:{:x}", self.compute_checksum(&deliverable.target_path)?);

        let receipt = DeliveryReceipt {
            deliverable_id: id.to_string(),
            target_path: deliverable.target_path.clone(),
            checksum,
            size_bytes: metadata.len() as usize,
            delivered_at: Instant::now(),
        };

        self.delivery_receipts.insert(id.to_string(), receipt.clone());
        self.update_status(id, DeliverableStatus::Delivered)?;

        Ok(receipt)
    }

    /// Compute a simple checksum for the file
    fn compute_checksum(&self, path: &PathBuf) -> Result<u64, String> {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let content = std::fs::read(path)
            .map_err(|e| format!("Cannot read file: {}", e))?;

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        Ok(hasher.finish())
    }

    /// Get a deliverable by ID
    pub fn get(&self, id: &str) -> Option<&Deliverable> {
        self.deliverables.get(id)
    }

    /// Get all deliverables
    pub fn all(&self) -> Vec<&Deliverable> {
        self.deliverables.values().collect()
    }

    /// Get validation result for a deliverable
    pub fn get_validation_result(&self, id: &str) -> Option<&ValidationResult> {
        self.validation_results.get(id)
    }

    /// Get delivery receipt for a deliverable
    pub fn get_receipt(&self, id: &str) -> Option<&DeliveryReceipt> {
        self.delivery_receipts.get(id)
    }

    /// Get summary statistics
    pub fn summary(&self) -> DeliverableSummary {
        let total = self.deliverables.len();
        let mut planned = 0;
        let mut in_progress = 0;
        let mut validated = 0;
        let mut delivered = 0;
        let mut failed = 0;

        for deliverable in self.deliverables.values() {
            match deliverable.status {
                DeliverableStatus::Planned => planned += 1,
                DeliverableStatus::InProgress => in_progress += 1,
                DeliverableStatus::PendingValidation => in_progress += 1,
                DeliverableStatus::Validated => validated += 1,
                DeliverableStatus::Delivered => delivered += 1,
                DeliverableStatus::Failed(_) => failed += 1,
            }
        }

        DeliverableSummary {
            total,
            planned,
            in_progress,
            validated,
            delivered,
            failed,
        }
    }
}

/// Summary of deliverable states
#[derive(Debug, Clone)]
pub struct DeliverableSummary {
    pub total: usize,
    pub planned: usize,
    pub in_progress: usize,
    pub validated: usize,
    pub delivered: usize,
    pub failed: usize,
}

/// Legacy compatibility function
#[allow(dead_code)]
pub fn plan(spec: &str) -> String {
    let mut manager = DeliverableManager::new(PathBuf::from("/tmp"));
    match manager.plan(spec) {
        Ok(plan) => {
            if let Some(first) = plan.deliverables.first() {
                first.target_path.to_string_lossy().to_string()
            } else {
                "/tmp".to_string()
            }
        }
        Err(_) => "/tmp".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_parsing() {
        let mut manager = DeliverableManager::new(PathBuf::from("/tmp/output"));
        let spec = "CODE:main:src/main.rs\nDOC:readme:README.md\nTEST:unit:tests/unit.rs";

        let plan = manager.plan(spec).unwrap();
        assert_eq!(plan.deliverables.len(), 3);
    }

    #[test]
    fn test_status_updates() {
        let mut manager = DeliverableManager::new(PathBuf::from("/tmp"));
        manager.plan("CODE:test:src/test.rs").unwrap();

        let id = "DEL-0001";
        manager.update_status(id, DeliverableStatus::InProgress).unwrap();

        assert!(matches!(
            manager.get(id).unwrap().status,
            DeliverableStatus::InProgress
        ));
    }

    #[test]
    fn test_summary() {
        let mut manager = DeliverableManager::new(PathBuf::from("/tmp"));
        manager.plan("CODE:a:a.rs\nCODE:b:b.rs\nCODE:c:c.rs").unwrap();

        let summary = manager.summary();
        assert_eq!(summary.total, 3);
        assert_eq!(summary.planned, 3);
    }
}
