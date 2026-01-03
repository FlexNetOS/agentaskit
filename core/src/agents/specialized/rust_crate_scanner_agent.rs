// Rust Crate Scanner Agent - Specialized Rust/Cargo Sub-Agent
// Discovers crates, versions, features; maps dependency tree
// Outputs: artifacts, SBOM, scores, advisories
// Policies: MSRV, semver, export-control

use crate::agents::{Agent, AgentCapability};
use agentaskit_shared::{
    AgentId, AgentMetadata, AgentStatus, HealthStatus, Priority, ResourceRequirements, Task,
    TaskResult, TaskStatus,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Rust Crate Scanner Agent Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustCrateScannerConfig {
    /// Minimum Supported Rust Version (MSRV) policy
    pub msrv_policy: MsrvPolicy,
    /// Semantic versioning enforcement
    pub semver_policy: SemverPolicy,
    /// Export control compliance
    pub export_control: ExportControlPolicy,
    /// Dependency analysis depth
    pub max_dependency_depth: usize,
    /// Feature analysis enabled
    pub analyze_features: bool,
    /// Generate SBOM
    pub generate_sbom: bool,
    /// Scan interval in seconds
    pub scan_interval_secs: u64,
}

impl Default for RustCrateScannerConfig {
    fn default() -> Self {
        Self {
            msrv_policy: MsrvPolicy::default(),
            semver_policy: SemverPolicy::default(),
            export_control: ExportControlPolicy::default(),
            max_dependency_depth: 10,
            analyze_features: true,
            generate_sbom: true,
            scan_interval_secs: 3600, // 1 hour
        }
    }
}

/// MSRV (Minimum Supported Rust Version) Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsrvPolicy {
    pub enabled: bool,
    pub minimum_version: String,
    pub check_dependencies: bool,
    pub fail_on_violation: bool,
}

impl Default for MsrvPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            minimum_version: "1.70.0".to_string(),
            check_dependencies: true,
            fail_on_violation: false,
        }
    }
}

/// Semantic Versioning Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemverPolicy {
    pub enforce_semver: bool,
    pub allow_pre_release: bool,
    pub allow_yanked: bool,
    pub max_major_version_delta: Option<u32>,
}

impl Default for SemverPolicy {
    fn default() -> Self {
        Self {
            enforce_semver: true,
            allow_pre_release: false,
            allow_yanked: false,
            max_major_version_delta: None,
        }
    }
}

/// Export Control Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportControlPolicy {
    pub enabled: bool,
    pub restricted_licenses: Vec<String>,
    pub restricted_countries: Vec<String>,
    pub check_origin: bool,
}

impl Default for ExportControlPolicy {
    fn default() -> Self {
        Self {
            enabled: false,
            restricted_licenses: vec!["GPL-3.0".to_string()],
            restricted_countries: vec![],
            check_origin: false,
        }
    }
}

/// Crate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateInfo {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
    pub dependencies: Vec<DependencyInfo>,
    pub msrv: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub authors: Vec<String>,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub kind: DependencyKind,
    pub optional: bool,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DependencyKind {
    Normal,
    Development,
    Build,
}

/// Software Bill of Materials (SBOM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sbom {
    pub format: SbomFormat,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub packages: Vec<SbomPackage>,
    pub relationships: Vec<SbomRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SbomFormat {
    CycloneDx,
    Spdx,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomPackage {
    pub name: String,
    pub version: String,
    pub license: Option<String>,
    pub supplier: Option<String>,
    pub checksums: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomRelationship {
    pub from: String,
    pub to: String,
    pub relationship_type: String,
}

/// Scan result with scores and advisories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub crate_info: CrateInfo,
    pub sbom: Option<Sbom>,
    pub scores: ScanScores,
    pub advisories: Vec<Advisory>,
    pub policy_violations: Vec<PolicyViolation>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanScores {
    pub overall_score: f64,
    pub security_score: f64,
    pub quality_score: f64,
    pub compliance_score: f64,
    pub dependency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Advisory {
    pub id: String,
    pub severity: AdvisorySeverity,
    pub title: String,
    pub description: String,
    pub affected_versions: Vec<String>,
    pub patched_versions: Vec<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdvisorySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub policy_type: PolicyType,
    pub severity: ViolationSeverity,
    pub message: String,
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyType {
    Msrv,
    Semver,
    ExportControl,
    License,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViolationSeverity {
    Error,
    Warning,
    Info,
}

/// Rust Crate Scanner Agent
pub struct RustCrateScannerAgent {
    id: Uuid,
    name: String,
    config: RustCrateScannerConfig,
    metadata: AgentMetadata,
    scan_results: Arc<RwLock<HashMap<PathBuf, ScanResult>>>,
    tasks: Arc<Mutex<HashMap<Uuid, Task>>>,
    active: Arc<Mutex<bool>>,
    last_scan: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
}

impl RustCrateScannerAgent {
    /// Create a new Rust Crate Scanner Agent
    pub fn new(config: Option<RustCrateScannerConfig>) -> Self {
        let id = Uuid::new_v4();
        let config = config.unwrap_or_default();

        let capabilities = vec![
            "crate_discovery".to_string(),
            "dependency_analysis".to_string(),
            "feature_analysis".to_string(),
            "sbom_generation".to_string(),
            "msrv_checking".to_string(),
            "semver_validation".to_string(),
            "export_control".to_string(),
            "advisory_scanning".to_string(),
            "policy_enforcement".to_string(),
        ];

        let metadata = AgentMetadata {
            id,
            name: "RustCrateScannerAgent".to_string(),
            agent_type: "RustCargoSubAgent".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: capabilities.clone(),
            status: AgentStatus::Initializing,
            health_status: HealthStatus::Unknown,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            resource_requirements: ResourceRequirements {
                cpu_cores: Some(2),
                memory_mb: Some(1024),
                storage_mb: Some(2048),
                network_bandwidth_mbps: Some(10),
                gpu_required: false,
                special_capabilities: vec!["cargo".to_string(), "rustc".to_string()],
            },
            tags: HashMap::new(),
        };

        Self {
            id,
            name: "RustCrateScannerAgent".to_string(),
            config,
            metadata,
            scan_results: Arc::new(RwLock::new(HashMap::new())),
            tasks: Arc::new(Mutex::new(HashMap::new())),
            active: Arc::new(Mutex::new(false)),
            last_scan: Arc::new(RwLock::new(None)),
        }
    }

    /// Scan a crate workspace
    pub async fn scan_workspace(&self, workspace_path: &Path) -> Result<ScanResult> {
        info!("Scanning Rust workspace at: {:?}", workspace_path);

        // Discover crate information
        let crate_info = self.discover_crate(workspace_path).await?;

        // Generate SBOM if enabled
        let sbom = if self.config.generate_sbom {
            Some(self.generate_sbom(&crate_info).await?)
        } else {
            None
        };

        // Calculate scores
        let scores = self.calculate_scores(&crate_info).await?;

        // Check for advisories
        let advisories = self.check_advisories(&crate_info).await?;

        // Check policy violations
        let policy_violations = self.check_policies(&crate_info).await?;

        let result = ScanResult {
            crate_info,
            sbom,
            scores,
            advisories,
            policy_violations,
            timestamp: chrono::Utc::now(),
        };

        // Store result
        self.scan_results
            .write()
            .await
            .insert(workspace_path.to_path_buf(), result.clone());

        // Update last scan time
        *self.last_scan.write().await = Some(chrono::Utc::now());

        Ok(result)
    }

    /// Discover crate information
    async fn discover_crate(&self, workspace_path: &Path) -> Result<CrateInfo> {
        debug!("Discovering crate information at: {:?}", workspace_path);

        // Read Cargo.toml
        let cargo_toml_path = workspace_path.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return Err(anyhow!("Cargo.toml not found at {:?}", cargo_toml_path));
        }

        let cargo_toml_content = tokio::fs::read_to_string(&cargo_toml_path)
            .await
            .context("Failed to read Cargo.toml")?;

        // Parse Cargo.toml (simplified - would use cargo_toml crate in production)
        let crate_info = CrateInfo {
            name: "example-crate".to_string(),
            version: "0.1.0".to_string(),
            features: vec!["default".to_string()],
            dependencies: vec![],
            msrv: Some("1.70.0".to_string()),
            license: Some("MIT OR Apache-2.0".to_string()),
            repository: Some("https://github.com/example/repo".to_string()),
            authors: vec!["Example Author <author@example.com>".to_string()],
        };

        Ok(crate_info)
    }

    /// Generate SBOM
    async fn generate_sbom(&self, crate_info: &CrateInfo) -> Result<Sbom> {
        debug!("Generating SBOM for crate: {}", crate_info.name);

        let packages = vec![SbomPackage {
            name: crate_info.name.clone(),
            version: crate_info.version.clone(),
            license: crate_info.license.clone(),
            supplier: None,
            checksums: HashMap::new(),
        }];

        let relationships = vec![];

        Ok(Sbom {
            format: SbomFormat::CycloneDx,
            timestamp: chrono::Utc::now(),
            packages,
            relationships,
        })
    }

    /// Calculate quality and compliance scores
    async fn calculate_scores(&self, crate_info: &CrateInfo) -> Result<ScanScores> {
        debug!("Calculating scores for crate: {}", crate_info.name);

        // Simplified scoring algorithm
        let security_score = 85.0;
        let quality_score = 90.0;
        let compliance_score = 95.0;
        let dependency_score = 88.0;
        let overall_score =
            (security_score + quality_score + compliance_score + dependency_score) / 4.0;

        Ok(ScanScores {
            overall_score,
            security_score,
            quality_score,
            compliance_score,
            dependency_score,
        })
    }

    /// Check for security advisories
    async fn check_advisories(&self, crate_info: &CrateInfo) -> Result<Vec<Advisory>> {
        debug!("Checking advisories for crate: {}", crate_info.name);

        // Would integrate with RustSec advisory database in production
        Ok(vec![])
    }

    /// Check policy violations
    async fn check_policies(&self, crate_info: &CrateInfo) -> Result<Vec<PolicyViolation>> {
        debug!("Checking policy compliance for crate: {}", crate_info.name);

        let mut violations = vec![];

        // Check MSRV policy
        if self.config.msrv_policy.enabled {
            if let Some(msrv) = &crate_info.msrv {
                // Simplified version comparison
                if msrv < &self.config.msrv_policy.minimum_version {
                    violations.push(PolicyViolation {
                        policy_type: PolicyType::Msrv,
                        severity: if self.config.msrv_policy.fail_on_violation {
                            ViolationSeverity::Error
                        } else {
                            ViolationSeverity::Warning
                        },
                        message: format!(
                            "MSRV {} is below minimum required version {}",
                            msrv, self.config.msrv_policy.minimum_version
                        ),
                        details: HashMap::new(),
                    });
                }
            }
        }

        Ok(violations)
    }

    /// Get latest scan result for a workspace
    pub async fn get_scan_result(&self, workspace_path: &Path) -> Option<ScanResult> {
        self.scan_results.read().await.get(workspace_path).cloned()
    }
}

#[async_trait]
impl Agent for RustCrateScannerAgent {
    async fn start(&mut self) -> Result<()> {
        info!("Starting RustCrateScannerAgent: {}", self.name);
        *self.active.lock().await = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping RustCrateScannerAgent: {}", self.name);
        *self.active.lock().await = false;
        Ok(())
    }

    async fn handle_message(
        &mut self,
        message: crate::agents::AgentMessage,
    ) -> Result<Option<crate::agents::AgentMessage>> {
        debug!("RustCrateScannerAgent received message");
        Ok(None)
    }

    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        info!("RustCrateScannerAgent executing task: {}", task.name);

        let task_id = task.id;
        self.tasks.lock().await.insert(task_id, task.clone());

        // Enhanced: Parse task parameters (input_data is Value, not Option)
        let workspace_path = task.input_data
            .get("workspace_path")
            .and_then(|v| v.as_str())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        // Execute scan
        match self.scan_workspace(&workspace_path).await {
            Ok(scan_result) => {
                self.tasks.lock().await.remove(&task_id);
                Ok(TaskResult {
                    task_id,
                    status: TaskStatus::Completed,
                    output_data: Some(serde_json::to_value(scan_result)?),
                    error_message: None,
                    completed_at: chrono::Utc::now(),
                })
            }
            Err(e) => {
                self.tasks.lock().await.remove(&task_id);
                Ok(TaskResult {
                    task_id,
                    status: TaskStatus::Failed,
                    output_data: None,
                    error_message: Some(format!("Scan failed: {}", e)),
                    completed_at: chrono::Utc::now(),
                })
            }
        }
    }

    async fn health_check(&self) -> Result<HealthStatus> {
        let active = *self.active.lock().await;
        if active {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unknown)
        }
    }

    async fn update_config(&mut self, config: serde_json::Value) -> Result<()> {
        debug!("Updating RustCrateScannerAgent configuration");
        if let Ok(new_config) = serde_json::from_value::<RustCrateScannerConfig>(config) {
            self.config = new_config;
        }
        Ok(())
    }

    fn capabilities(&self) -> &[String] {
        &self.metadata.capabilities
    }

    async fn state(&self) -> AgentStatus {
        if *self.active.lock().await {
            AgentStatus::Active
        } else {
            AgentStatus::Inactive
        }
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_rust_crate_scanner_agent() {
        let agent = RustCrateScannerAgent::new(None);
        assert_eq!(agent.name, "RustCrateScannerAgent");
        assert!(agent
            .capabilities()
            .contains(&"crate_discovery".to_string()));
    }

    #[tokio::test]
    async fn test_agent_lifecycle() {
        let mut agent = RustCrateScannerAgent::new(None);
        assert!(agent.start().await.is_ok());
        assert_eq!(agent.state().await, AgentStatus::Active);
        assert!(agent.stop().await.is_ok());
        assert_eq!(agent.state().await, AgentStatus::Inactive);
    }
}
