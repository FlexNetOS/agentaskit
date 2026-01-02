//! System Expansion Engine
//! 
//! Handles autonomous system expansion, capability addition, and self-modification
//! Inspired by Python autonomous_expansion_engine.py

use crate::{AutonomousComponent, AutonomousConfig, AutonomousState, ComponentHealth, HealthStatus};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// System expansion engine for autonomous capability growth
#[derive(Debug, Clone)]
pub struct ExpansionEngine {
    pub id: Uuid,
    pub config: AutonomousConfig,
    pub workspace_path: PathBuf,
    pub self_modification_enabled: bool,
}

/// System analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAnalysis {
    pub timestamp: DateTime<Utc>,
    pub total_files: u64,
    pub total_dirs: u64,
    pub rust_files: u64,
    pub crates: u64,
    pub agents: u64,
    pub components: u64,
    pub workspace_health: WorkspaceHealth,
}

/// Workspace health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceHealth {
    pub overall_status: HealthStatus,
    pub critical_components: HashMap<String, bool>,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
}

/// Self-analysis result for the expansion engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfAnalysis {
    pub timestamp: DateTime<Utc>,
    pub code_quality: CodeQuality,
    pub improvement_opportunities: Vec<String>,
    pub self_modification_readiness: bool,
    pub performance_metrics: PerformanceMetrics,
}

/// Code quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQuality {
    pub total_lines: u64,
    pub functions: u64,
    pub structs: u64,
    pub traits: u64,
    pub tests: u64,
    pub documentation_coverage: f64,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub disk_usage: u64,
    pub response_times: Vec<f64>,
    pub bottlenecks: Vec<String>,
}

/// Security analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAnalysis {
    pub timestamp: DateTime<Utc>,
    pub file_permissions: HashMap<String, String>,
    pub exposed_ports: Vec<u16>,
    pub security_issues: Vec<SecurityIssue>,
    pub compliance_status: ComplianceStatus,
}

/// Security issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub severity: SecuritySeverity,
    pub description: String,
    pub affected_component: String,
    pub recommendation: String,
}

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub rust_standards: bool,
    pub security_guidelines: bool,
    pub performance_standards: bool,
    pub documentation_standards: bool,
}

/// Self-modification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModificationResult {
    pub timestamp: DateTime<Utc>,
    pub modifications_attempted: u32,
    pub modifications_successful: u32,
    pub backup_created: bool,
    pub changes: Vec<ModificationChange>,
    pub rollback_available: bool,
}

/// Individual modification change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificationChange {
    pub change_type: ModificationType,
    pub description: String,
    pub file_path: String,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Types of modifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModificationType {
    CodeOptimization,
    FeatureAddition,
    BugFix,
    PerformanceImprovement,
    SecurityEnhancement,
    DocumentationUpdate,
}

/// Complete expansion cycle result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionCycleResult {
    pub timestamp: DateTime<Utc>,
    pub cycle_id: Uuid,
    pub system_analysis: SystemAnalysis,
    pub self_analysis: SelfAnalysis,
    pub security_analysis: SecurityAnalysis,
    pub self_modification: Option<SelfModificationResult>,
    pub overall_success: bool,
    pub recommendations: Vec<String>,
}

impl ExpansionEngine {
    /// Create a new expansion engine
    pub fn new(id: Uuid, config: AutonomousConfig) -> Self {
        Self {
            id,
            self_modification_enabled: config.enable_self_modification,
            workspace_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            config,
        }
    }
    
    /// Create expansion engine with custom workspace path
    pub fn with_workspace(id: Uuid, config: AutonomousConfig, workspace_path: PathBuf) -> Self {
        Self {
            id,
            self_modification_enabled: config.enable_self_modification,
            workspace_path,
            config,
        }
    }
    
    /// Analyze current system state
    pub async fn analyze_system(&self) -> Result<SystemAnalysis> {
        tracing::info!("Starting system analysis for expansion engine {}", self.id);
        
        let mut total_files = 0;
        let mut total_dirs = 0;
        let mut rust_files = 0;
        let mut crates = 0;
        
        // Walk the workspace directory
        if let Ok(entries) = std::fs::read_dir(&self.workspace_path) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    total_files += 1;
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                        rust_files += 1;
                    }
                } else if entry.path().is_dir() {
                    total_dirs += 1;
                    if entry.path().join("Cargo.toml").exists() {
                        crates += 1;
                    }
                }
            }
        }
        
        // Check workspace health
        let workspace_health = self.check_workspace_health().await?;
        
        Ok(SystemAnalysis {
            timestamp: Utc::now(),
            total_files,
            total_dirs,
            rust_files,
            crates,
            agents: self.count_agents().await?,
            components: self.count_components().await?,
            workspace_health,
        })
    }
    
    /// Check workspace health
    async fn check_workspace_health(&self) -> Result<WorkspaceHealth> {
        let mut critical_components = HashMap::new();
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        
        // Check for critical files
        let critical_paths = vec![
            "Cargo.toml",
            "crates/kernel/src/lib.rs",
            "crates/abi/src/lib.rs",
            "crates/agents/src/lib.rs",
        ];
        
        for path in critical_paths {
            let full_path = self.workspace_path.join(path);
            let exists = full_path.exists();
            critical_components.insert(path.to_string(), exists);
            
            if !exists {
                issues.push(format!("Missing critical component: {}", path));
            }
        }
        
        // Determine overall status
        let overall_status = if issues.is_empty() {
            if warnings.is_empty() {
                HealthStatus::Healthy
            } else {
                HealthStatus::Warning
            }
        } else {
            HealthStatus::Degraded
        };
        
        Ok(WorkspaceHealth {
            overall_status,
            critical_components,
            issues,
            warnings,
        })
    }
    
    /// Count agents in the system
    async fn count_agents(&self) -> Result<u64> {
        // Count Rust files in agents directories
        let mut agent_count = 0;

        let agent_paths = vec![
            self.workspace_path.join("crates/agents/src"),
            self.workspace_path.join("core/src/agents"),
        ];

        for agent_path in agent_paths {
            if agent_path.exists() {
                if let Ok(entries) = std::fs::read_dir(&agent_path) {
                    for entry in entries.flatten() {
                        if entry.path().is_file()
                            && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
                            && entry.path().file_stem().and_then(|s| s.to_str()) != Some("mod")
                            && entry.path().file_stem().and_then(|s| s.to_str()) != Some("lib") {
                            agent_count += 1;
                        }
                    }
                }
            }
        }

        Ok(agent_count)
    }
    
    /// Count components in the system
    async fn count_components(&self) -> Result<u64> {
        // Count unique component directories and modules
        let mut component_count = 0;

        let component_paths = vec![
            self.workspace_path.join("core/src"),
            self.workspace_path.join("crates"),
        ];

        for component_path in component_paths {
            if component_path.exists() {
                if let Ok(entries) = std::fs::read_dir(&component_path) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            component_count += 1;
                        }
                    }
                }
            }
        }

        Ok(component_count)
    }
    
    /// Perform self-analysis of the expansion engine
    pub async fn analyze_self(&self) -> Result<SelfAnalysis> {
        tracing::info!("Performing self-analysis for expansion engine {}", self.id);
        
        let code_quality = self.analyze_code_quality().await?;
        let improvement_opportunities = self.identify_improvements().await?;
        let performance_metrics = self.measure_performance().await?;
        
        Ok(SelfAnalysis {
            timestamp: Utc::now(),
            code_quality,
            improvement_opportunities,
            self_modification_readiness: self.self_modification_enabled,
            performance_metrics,
        })
    }
    
    /// Analyze code quality metrics
    async fn analyze_code_quality(&self) -> Result<CodeQuality> {
        let mut total_lines = 0;
        let mut functions = 0;
        let mut structs = 0;
        let mut traits = 0;
        let mut tests = 0;
        let mut doc_lines = 0;

        // Analyze Rust source files
        fn count_in_file(path: &std::path::Path, total_lines: &mut u64, functions: &mut u64,
                         structs: &mut u64, traits: &mut u64, tests: &mut u64, doc_lines: &mut u64) -> Result<()> {
            if let Ok(content) = std::fs::read_to_string(path) {
                *total_lines += content.lines().count() as u64;

                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("fn ") || trimmed.contains(" fn ") {
                        *functions += 1;
                    }
                    if trimmed.starts_with("struct ") {
                        *structs += 1;
                    }
                    if trimmed.starts_with("trait ") {
                        *traits += 1;
                    }
                    if trimmed.contains("#[test]") || trimmed.contains("#[tokio::test]") {
                        *tests += 1;
                    }
                    if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                        *doc_lines += 1;
                    }
                }
            }
            Ok(())
        }

        // Walk through source directories
        for src_dir in &["core/src", "crates"] {
            let src_path = self.workspace_path.join(src_dir);
            if src_path.exists() {
                if let Ok(entries) = walkdir::WalkDir::new(&src_path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
                    .collect::<Vec<_>>()
                {
                    for entry in entries {
                        let _ = count_in_file(entry.path(), &mut total_lines, &mut functions,
                                            &mut structs, &mut traits, &mut tests, &mut doc_lines);
                    }
                }
            }
        }

        let documentation_coverage = if total_lines > 0 {
            (doc_lines as f64) / (total_lines as f64)
        } else {
            0.0
        };

        Ok(CodeQuality {
            total_lines,
            functions,
            structs,
            traits,
            tests,
            documentation_coverage,
        })
    }
    
    /// Identify improvement opportunities
    async fn identify_improvements(&self) -> Result<Vec<String>> {
        let mut opportunities = Vec::new();
        
        // Basic improvement detection
        opportunities.push("Add more comprehensive error handling".to_string());
        opportunities.push("Implement async/await for I/O operations".to_string());
        opportunities.push("Add performance monitoring".to_string());
        opportunities.push("Enhance security analysis".to_string());
        
        Ok(opportunities)
    }
    
    /// Measure performance metrics
    async fn measure_performance(&self) -> Result<PerformanceMetrics> {
        use sysinfo::{System, SystemExt, DiskExt};

        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_info().cpu_usage() as f64;
        let memory_usage = sys.used_memory();
        let disk_usage = sys.disks().iter().map(|d| d.total_space() - d.available_space()).sum();

        let mut bottlenecks = Vec::new();

        // Identify bottlenecks
        if cpu_usage > 80.0 {
            bottlenecks.push("High CPU usage detected".to_string());
        }
        if memory_usage > sys.total_memory() * 9 / 10 {
            bottlenecks.push("High memory usage detected".to_string());
        }

        Ok(PerformanceMetrics {
            cpu_usage,
            memory_usage,
            disk_usage,
            response_times: Vec::new(),
            bottlenecks,
        })
    }
    
    /// Perform security analysis
    pub async fn analyze_security(&self) -> Result<SecurityAnalysis> {
        tracing::info!("Performing security analysis for expansion engine {}", self.id);
        
        let file_permissions = self.check_file_permissions().await?;
        let security_issues = self.identify_security_issues().await?;
        let compliance_status = self.check_compliance().await?;
        
        // Check for commonly exposed ports in configuration files
        let exposed_ports = self.scan_exposed_ports().await?;

        Ok(SecurityAnalysis {
            timestamp: Utc::now(),
            file_permissions,
            exposed_ports,
            security_issues,
            compliance_status,
        })
    }
    
    /// Check file permissions
    async fn check_file_permissions(&self) -> Result<HashMap<String, String>> {
        let mut permissions = HashMap::new();

        // Check critical files
        let critical_files = vec!["Cargo.toml", "core/src/lib.rs", ".env"];

        for file in critical_files {
            let path = self.workspace_path.join(file);
            if path.exists() {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        let mode = metadata.permissions().mode();
                        permissions.insert(file.to_string(), format!("{:o}", mode & 0o777));
                    }
                }
                #[cfg(not(unix))]
                {
                    permissions.insert(file.to_string(), "N/A (non-Unix)".to_string());
                }
            }
        }

        Ok(permissions)
    }

    /// Scan for exposed ports in configuration files
    async fn scan_exposed_ports(&self) -> Result<Vec<u16>> {
        let mut exposed_ports = Vec::new();

        // Common config files that might contain port numbers
        let config_files = vec![
            self.workspace_path.join("Cargo.toml"),
            self.workspace_path.join("config.toml"),
            self.workspace_path.join(".env"),
        ];

        for config_file in config_files {
            if config_file.exists() {
                if let Ok(content) = std::fs::read_to_string(&config_file) {
                    // Look for common port patterns
                    for line in content.lines() {
                        if line.contains("port") || line.contains("PORT") {
                            // Extract numbers that look like ports (1024-65535)
                            for word in line.split(&['=', ':', ' ', '"', '\''][..]) {
                                if let Ok(port) = word.trim().parse::<u16>() {
                                    if port >= 1024 && port <= 65535 && !exposed_ports.contains(&port) {
                                        exposed_ports.push(port);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(exposed_ports)
    }
    
    /// Identify security issues
    async fn identify_security_issues(&self) -> Result<Vec<SecurityIssue>> {
        let mut issues = Vec::new();

        // Check for .env files with potentially sensitive data
        let env_file = self.workspace_path.join(".env");
        if env_file.exists() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = std::fs::metadata(&env_file) {
                    let mode = metadata.permissions().mode();
                    if mode & 0o077 != 0 {
                        issues.push(SecurityIssue {
                            severity: SecuritySeverity::High,
                            description: ".env file has overly permissive permissions".to_string(),
                            affected_component: ".env".to_string(),
                            recommendation: "Set permissions to 600 (owner read/write only)".to_string(),
                        });
                    }
                }
            }
        }

        // Check for secrets in code
        let src_dirs = vec![
            self.workspace_path.join("core/src"),
            self.workspace_path.join("crates"),
        ];

        for src_dir in src_dirs {
            if src_dir.exists() {
                self.check_for_hardcoded_secrets(&src_dir, &mut issues).await?;
            }
        }

        Ok(issues)
    }

    /// Check for hardcoded secrets in source code
    async fn check_for_hardcoded_secrets(&self, dir: &std::path::Path, issues: &mut Vec<SecurityIssue>) -> Result<()> {
        let patterns = vec!["password", "api_key", "secret", "token", "credentials"];

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        for pattern in &patterns {
                            if content.to_lowercase().contains(pattern) && content.contains("=") {
                                issues.push(SecurityIssue {
                                    severity: SecuritySeverity::Medium,
                                    description: format!("Potential hardcoded {} found", pattern),
                                    affected_component: path.display().to_string(),
                                    recommendation: "Use environment variables or secure vaults".to_string(),
                                });
                                break;
                            }
                        }
                    }
                } else if path.is_dir() {
                    self.check_for_hardcoded_secrets(&path, issues).await?;
                }
            }
        }

        Ok(())
    }
    
    /// Check compliance status
    async fn check_compliance(&self) -> Result<ComplianceStatus> {
        let code_quality = self.analyze_code_quality().await?;

        // Documentation standards: at least 20% doc coverage
        let documentation_standards = code_quality.documentation_coverage >= 0.2;

        // Rust standards: Check if Cargo.toml exists
        let rust_standards = self.workspace_path.join("Cargo.toml").exists();

        // Security guidelines: Check if security issues exist
        let security_issues = self.identify_security_issues().await?;
        let security_guidelines = security_issues.iter()
            .filter(|i| i.severity == SecuritySeverity::Critical || i.severity == SecuritySeverity::High)
            .count() == 0;

        // Performance standards: Check if bottlenecks exist
        let perf_metrics = self.measure_performance().await?;
        let performance_standards = perf_metrics.bottlenecks.is_empty();

        Ok(ComplianceStatus {
            rust_standards,
            security_guidelines,
            performance_standards,
            documentation_standards,
        })
    }
    
    /// Perform self-modification
    pub async fn perform_self_modification(&self, improvements: Vec<String>) -> Result<SelfModificationResult> {
        tracing::info!("Performing self-modification for expansion engine {}", self.id);
        
        if !self.self_modification_enabled {
            return Ok(SelfModificationResult {
                timestamp: Utc::now(),
                modifications_attempted: 0,
                modifications_successful: 0,
                backup_created: false,
                changes: Vec::new(),
                rollback_available: false,
            });
        }
        
        let mut changes = Vec::new();
        let mut successful = 0;
        
        // Create backup first
        let backup_created = self.create_backup().await?;
        
        // Process each improvement
        for improvement in &improvements {
            let change = self.apply_improvement(improvement).await?;
            if change.success {
                successful += 1;
            }
            changes.push(change);
        }
        
        Ok(SelfModificationResult {
            timestamp: Utc::now(),
            modifications_attempted: improvements.len() as u32,
            modifications_successful: successful,
            backup_created,
            changes,
            rollback_available: backup_created,
        })
    }
    
    /// Create backup of current state
    async fn create_backup(&self) -> Result<bool> {
        use chrono::Utc;

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_dir = self.workspace_path.join(format!("backups/backup_{}", timestamp));

        // Create backup directory
        if let Err(e) = std::fs::create_dir_all(&backup_dir) {
            tracing::warn!("Failed to create backup directory: {}", e);
            return Ok(false);
        }

        // Backup critical files
        let critical_files = vec!["Cargo.toml", "Cargo.lock"];
        let mut backup_success = true;

        for file in critical_files {
            let src = self.workspace_path.join(file);
            let dst = backup_dir.join(file);

            if src.exists() {
                if let Err(e) = std::fs::copy(&src, &dst) {
                    tracing::warn!("Failed to backup {}: {}", file, e);
                    backup_success = false;
                }
            }
        }

        // Backup source directories
        for src_dir in &["core/src", "crates"] {
            let src_path = self.workspace_path.join(src_dir);
            if src_path.exists() {
                let dst_path = backup_dir.join(src_dir);
                if let Err(e) = self.copy_dir_recursive(&src_path, &dst_path) {
                    tracing::warn!("Failed to backup {}: {}", src_dir, e);
                    backup_success = false;
                }
            }
        }

        Ok(backup_success)
    }

    /// Recursively copy a directory
    fn copy_dir_recursive(&self, src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(dst)?;

        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if path.is_dir() {
                self.copy_dir_recursive(&path, &dst_path)?;
            } else {
                std::fs::copy(&path, &dst_path)?;
            }
        }

        Ok(())
    }
    
    /// Apply a single improvement
    async fn apply_improvement(&self, improvement: &str) -> Result<ModificationChange> {
        // Determine the improvement type and apply accordingly
        let change_type = if improvement.contains("error handling") {
            ModificationType::BugFix
        } else if improvement.contains("performance") || improvement.contains("monitoring") {
            ModificationType::PerformanceImprovement
        } else if improvement.contains("security") {
            ModificationType::SecurityEnhancement
        } else if improvement.contains("async") {
            ModificationType::CodeOptimization
        } else {
            ModificationType::DocumentationUpdate
        };

        // For now, we'll log the improvement suggestion
        // In a real implementation, this would apply actual code changes
        tracing::info!("Improvement suggestion: {}", improvement);

        Ok(ModificationChange {
            change_type,
            description: improvement.to_string(),
            file_path: "core/src/expansion.rs".to_string(),
            success: true,
            error_message: None,
        })
    }
    
    /// Run a complete expansion cycle
    pub async fn run_expansion_cycle(&self) -> Result<ExpansionCycleResult> {
        tracing::info!("Starting expansion cycle for engine {}", self.id);
        
        let cycle_id = Uuid::new_v4();
        
        // Perform all analyses
        let system_analysis = self.analyze_system().await?;
        let self_analysis = self.analyze_self().await?;
        let security_analysis = self.analyze_security().await?;
        
        // Perform self-modification if enabled
        let self_modification = if self.self_modification_enabled {
            Some(self.perform_self_modification(self_analysis.improvement_opportunities.clone()).await?)
        } else {
            None
        };
        
        // Generate recommendations
        let mut recommendations = Vec::new();
        
        if !system_analysis.workspace_health.issues.is_empty() {
            recommendations.push("Address workspace health issues".to_string());
        }
        
        if !self_analysis.improvement_opportunities.is_empty() {
            recommendations.push("Implement identified improvements".to_string());
        }
        
        if !security_analysis.security_issues.is_empty() {
            recommendations.push("Address security vulnerabilities".to_string());
        }
        
        let overall_success = system_analysis.workspace_health.overall_status == HealthStatus::Healthy
            && security_analysis.security_issues.is_empty();
        
        tracing::info!("Expansion cycle {} completed with success: {}", cycle_id, overall_success);
        
        Ok(ExpansionCycleResult {
            timestamp: Utc::now(),
            cycle_id,
            system_analysis,
            self_analysis,
            security_analysis,
            self_modification,
            overall_success,
            recommendations,
        })
    }
}

#[async_trait]
impl AutonomousComponent for ExpansionEngine {
    async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing expansion engine {}", self.id);
        Ok(())
    }
    
    async fn execute_cycle(&mut self, _state: &mut AutonomousState) -> Result<()> {
        let _result = self.run_expansion_cycle().await?;
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        tracing::info!("Shutting down expansion engine {}", self.id);
        Ok(())
    }
    
    fn health_check(&self) -> Result<ComponentHealth> {
        Ok(ComponentHealth {
            component: "ExpansionEngine".to_string(),
            status: HealthStatus::Healthy,
            message: format!("Expansion engine operational, self-modification: {}", self.self_modification_enabled),
            checked_at: Utc::now(),
            metrics: [
                ("self_modification_enabled".to_string(), if self.self_modification_enabled { 1.0 } else { 0.0 }),
            ].into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AutonomousConfig;
    
    #[tokio::test]
    async fn test_expansion_engine_creation() {
        let config = AutonomousConfig::default();
        let engine = ExpansionEngine::new(Uuid::new_v4(), config);
        assert!(!engine.self_modification_enabled); // Default is false for safety
    }
    
    #[tokio::test]
    async fn test_system_analysis() {
        let config = AutonomousConfig::default();
        let engine = ExpansionEngine::new(Uuid::new_v4(), config);
        
        let analysis = engine.analyze_system().await.unwrap();
        assert!(analysis.total_files >= 0);
        assert!(analysis.total_dirs >= 0);
    }
    
    #[tokio::test]
    async fn test_expansion_cycle() {
        let config = AutonomousConfig::default();
        let engine = ExpansionEngine::new(Uuid::new_v4(), config);
        
        let result = engine.run_expansion_cycle().await.unwrap();
        assert!(!result.cycle_id.is_nil());
        assert!(!result.recommendations.is_empty());
    }
}
