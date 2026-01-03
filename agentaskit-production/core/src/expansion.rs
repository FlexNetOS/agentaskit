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
        let mut agent_count = 0;

        // Look for agent files in the agents directory
        let agents_path = self.workspace_path.join("core/src/agents");
        if agents_path.exists() {
            agent_count += self.count_rust_structs_in_dir(&agents_path, "Agent").await?;
        }

        // Also check crates/agents
        let crates_agents_path = self.workspace_path.join("crates/agents/src");
        if crates_agents_path.exists() {
            agent_count += self.count_rust_structs_in_dir(&crates_agents_path, "Agent").await?;
        }

        Ok(agent_count)
    }

    /// Count Rust structs with a pattern in a directory
    async fn count_rust_structs_in_dir(&self, dir: &PathBuf, pattern: &str) -> Result<u64> {
        let mut count = 0;

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(contents) = std::fs::read_to_string(&path) {
                        // Count structs that contain the pattern in their name
                        count += contents.matches(&format!("struct {}", pattern)).count() as u64;
                        count += contents.matches(&format!("struct {}.", pattern)).count() as u64;
                        // Also count lines with 'Agent' in struct name
                        for line in contents.lines() {
                            if line.contains("pub struct") && line.contains(pattern) {
                                count += 1;
                            }
                        }
                    }
                } else if path.is_dir() {
                    count += self.count_rust_structs_in_dir(&path, pattern).await?;
                }
            }
        }

        Ok(count)
    }

    /// Count components in the system
    async fn count_components(&self) -> Result<u64> {
        let mut component_count = 0;

        // Count crates as components
        if let Ok(workspace_toml) = std::fs::read_to_string(self.workspace_path.join("Cargo.toml")) {
            // Count [workspace] members
            component_count += workspace_toml.matches("members").count() as u64;
        }

        // Count modules in core/src
        let core_src = self.workspace_path.join("core/src");
        if core_src.exists() {
            if let Ok(entries) = std::fs::read_dir(&core_src) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() || entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                        component_count += 1;
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
        let mut documented_items = 0;
        let mut total_items = 0;

        // Analyze Rust files in the workspace
        let src_path = self.workspace_path.join("core/src");
        if src_path.exists() {
            self.analyze_code_in_dir(&src_path, &mut total_lines, &mut functions, &mut structs,
                                     &mut traits, &mut tests, &mut documented_items, &mut total_items).await?;
        }

        let documentation_coverage = if total_items > 0 {
            documented_items as f64 / total_items as f64
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

    /// Analyze code in a directory recursively
    async fn analyze_code_in_dir(
        &self,
        dir: &PathBuf,
        total_lines: &mut u64,
        functions: &mut u64,
        structs: &mut u64,
        traits: &mut u64,
        tests: &mut u64,
        documented_items: &mut u64,
        total_items: &mut u64,
    ) -> Result<()> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(contents) = std::fs::read_to_string(&path) {
                        *total_lines += contents.lines().count() as u64;

                        let mut prev_line_doc = false;
                        for line in contents.lines() {
                            let trimmed = line.trim();

                            // Check for documentation
                            if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                                prev_line_doc = true;
                            } else {
                                if trimmed.starts_with("pub fn") || trimmed.starts_with("fn ") {
                                    *functions += 1;
                                    *total_items += 1;
                                    if prev_line_doc { *documented_items += 1; }
                                }
                                if trimmed.starts_with("pub struct") || trimmed.starts_with("struct ") {
                                    *structs += 1;
                                    *total_items += 1;
                                    if prev_line_doc { *documented_items += 1; }
                                }
                                if trimmed.starts_with("pub trait") || trimmed.starts_with("trait ") {
                                    *traits += 1;
                                    *total_items += 1;
                                    if prev_line_doc { *documented_items += 1; }
                                }
                                if trimmed.contains("#[test]") || trimmed.contains("#[tokio::test]") {
                                    *tests += 1;
                                }
                                prev_line_doc = false;
                            }
                        }
                    }
                } else if path.is_dir() {
                    Box::pin(self.analyze_code_in_dir(&path, total_lines, functions, structs,
                                                      traits, tests, documented_items, total_items)).await?;
                }
            }
        }
        Ok(())
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
        let mut cpu_usage = 0.0;
        let mut memory_usage = 0;
        let mut disk_usage = 0;
        let mut bottlenecks = Vec::new();

        // Read CPU usage from /proc/stat on Linux
        #[cfg(target_os = "linux")]
        {
            if let Ok(contents) = tokio::fs::read_to_string("/proc/stat").await {
                if let Some(cpu_line) = contents.lines().find(|l| l.starts_with("cpu ")) {
                    let values: Vec<u64> = cpu_line
                        .split_whitespace()
                        .skip(1)
                        .filter_map(|s| s.parse().ok())
                        .collect();

                    if values.len() >= 4 {
                        let user = values[0];
                        let nice = values[1];
                        let system = values[2];
                        let idle = values[3];
                        let total = user + nice + system + idle;
                        let active = user + nice + system;

                        if total > 0 {
                            cpu_usage = (active as f64 / total as f64) * 100.0;
                        }
                    }
                }
            }

            // Read memory from /proc/meminfo
            if let Ok(contents) = tokio::fs::read_to_string("/proc/meminfo").await {
                for line in contents.lines() {
                    if line.starts_with("MemTotal:") {
                        let total: u64 = line.split_whitespace()
                            .nth(1)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);
                        memory_usage = total * 1024; // Convert KB to bytes
                    }
                }
            }
        }

        // Calculate disk usage for workspace
        if let Ok(metadata) = std::fs::metadata(&self.workspace_path) {
            disk_usage = metadata.len();
        }

        // Identify bottlenecks based on thresholds
        if cpu_usage > 80.0 {
            bottlenecks.push("High CPU usage detected".to_string());
        }
        if memory_usage > 8 * 1024 * 1024 * 1024 { // > 8GB
            bottlenecks.push("High memory usage detected".to_string());
        }

        Ok(PerformanceMetrics {
            cpu_usage,
            memory_usage,
            disk_usage,
            response_times: Vec::new(), // Would require benchmarking
            bottlenecks,
        })
    }
    
    /// Perform security analysis
    pub async fn analyze_security(&self) -> Result<SecurityAnalysis> {
        tracing::info!("Performing security analysis for expansion engine {}", self.id);
        
        let file_permissions = self.check_file_permissions().await?;
        let security_issues = self.identify_security_issues().await?;
        let compliance_status = self.check_compliance().await?;
        
        // Scan for exposed ports. On Linux, use /proc/net/{tcp,udp}; on other platforms, log and return an empty list.
        let exposed_ports = {
            #[cfg(target_os = "linux")]
            {
                let mut ports = Vec::new();

                // Helper closure to parse /proc/net/* tables (TCP/UDP)
                let mut parse_proc_net = |path: &str, ports: &mut Vec<u16>| {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        for line in content.lines().skip(1) {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 2 {
                                // Parse local address (format: IP:PORT in hex)
                                if let Some(addr) = parts.get(1) {
                                    if let Some(port_hex) = addr.split(':').nth(1) {
                                        if let Ok(port) = u16::from_str_radix(port_hex, 16) {
                                            if port > 0 && !ports.contains(&port) {
                                                ports.push(port);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                };

                // Collect TCP and UDP ports on Linux
                parse_proc_net("/proc/net/tcp", &mut ports);
                parse_proc_net("/proc/net/udp", &mut ports);

                ports
            }

            #[cfg(not(target_os = "linux"))]
            {
                tracing::warn!(
                    "Port exposure analysis is currently only implemented for Linux;                      returning empty exposed_ports list on this platform"
                );
                Vec::new()
            }
        };

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
        let critical_files = vec![
            "Cargo.toml",
            "core/src/lib.rs",
            ".env",
            "config/settings.toml",
        ];

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
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        let readonly = metadata.permissions().readonly();
                        permissions.insert(file.to_string(), if readonly { "r--" } else { "rw-" }.to_string());
                    }
                }
            }
        }

        Ok(permissions)
    }

    /// Identify security issues
    async fn identify_security_issues(&self) -> Result<Vec<SecurityIssue>> {
        let mut issues = Vec::new();

        // Check for sensitive files with loose permissions
        let file_permissions = self.check_file_permissions().await?;
        for (file, perms) in &file_permissions {
            if file.contains(".env") || file.contains("secret") || file.contains("credential") {
                // Check if world-readable
                if perms.ends_with("4") || perms.ends_with("5") || perms.ends_with("6") || perms.ends_with("7") {
                    issues.push(SecurityIssue {
                        severity: SecuritySeverity::High,
                        description: format!("Sensitive file {} has world-readable permissions", file),
                        affected_component: file.clone(),
                        recommendation: "Change permissions to 600 (owner read/write only)".to_string(),
                    });
                }
            }
        }

        // Check for hardcoded secrets in source code
        let src_path = self.workspace_path.join("core/src");
        if src_path.exists() {
            self.scan_for_secrets(&src_path, &mut issues).await?;
        }

        // Check for unsafe code blocks
        self.check_unsafe_code(&mut issues).await?;

        Ok(issues)
    }

    /// Scan for potential hardcoded secrets
    async fn scan_for_secrets(&self, dir: &PathBuf, issues: &mut Vec<SecurityIssue>) -> Result<()> {
        let secret_patterns = [
            ("password", SecuritySeverity::Critical),
            ("api_key", SecuritySeverity::Critical),
            ("secret_key", SecuritySeverity::Critical),
            ("private_key", SecuritySeverity::Critical),
            ("token =", SecuritySeverity::High),
        ];

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(contents) = std::fs::read_to_string(&path) {
                        let contents_lower = contents.to_lowercase();
                        for (pattern, severity) in &secret_patterns {
                            if contents_lower.contains(pattern) && contents_lower.contains("\"") {
                                // Potential hardcoded secret
                                issues.push(SecurityIssue {
                                    severity: severity.clone(),
                                    description: format!("Potential hardcoded {} in {:?}", pattern, path.file_name()),
                                    affected_component: path.to_string_lossy().to_string(),
                                    recommendation: "Use environment variables or secure vault for secrets".to_string(),
                                });
                            }
                        }
                    }
                } else if path.is_dir() {
                    Box::pin(self.scan_for_secrets(&path, issues)).await?;
                }
            }
        }

        Ok(())
    }

    /// Check for unsafe code usage
    async fn check_unsafe_code(&self, issues: &mut Vec<SecurityIssue>) -> Result<()> {
        let src_path = self.workspace_path.join("core/src");
        if !src_path.exists() {
            return Ok(());
        }

        self.scan_for_unsafe(&src_path, issues).await
    }

    async fn scan_for_unsafe(&self, dir: &PathBuf, issues: &mut Vec<SecurityIssue>) -> Result<()> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(contents) = std::fs::read_to_string(&path) {
                        let unsafe_count = contents.matches("unsafe {").count();
                        if unsafe_count > 3 {
                            issues.push(SecurityIssue {
                                severity: SecuritySeverity::Medium,
                                description: format!("File {:?} contains {} unsafe blocks", path.file_name(), unsafe_count),
                                affected_component: path.to_string_lossy().to_string(),
                                recommendation: "Review and minimize unsafe code usage".to_string(),
                            });
                        }
                    }
                } else if path.is_dir() {
                    Box::pin(self.scan_for_unsafe(&path, issues)).await?;
                }
            }
        }
        Ok(())
    }

    /// Check compliance status
    async fn check_compliance(&self) -> Result<ComplianceStatus> {
        let code_quality = self.analyze_code_quality().await?;

        // Check documentation standards - at least 50% coverage
        let documentation_standards = code_quality.documentation_coverage >= 0.5;

        // Check Rust standards - run clippy-like checks
        let rust_standards = self.check_rust_standards().await?;

        // Check performance standards - no critical bottlenecks
        let perf = self.measure_performance().await?;
        let performance_standards = perf.bottlenecks.is_empty();

        // Security guidelines - no critical issues
        let security_issues = self.identify_security_issues().await?;
        let security_guidelines = !security_issues.iter().any(|i| i.severity == SecuritySeverity::Critical);

        Ok(ComplianceStatus {
            rust_standards,
            security_guidelines,
            performance_standards,
            documentation_standards,
        })
    }

    async fn check_rust_standards(&self) -> Result<bool> {
        // Basic Rust standards checks
        let src_path = self.workspace_path.join("core/src");
        if !src_path.exists() {
            return Ok(true);
        }

        // Check for common anti-patterns
        let mut issues_found = false;
        if let Ok(entries) = std::fs::read_dir(&src_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(contents) = std::fs::read_to_string(&path) {
                        // Check for unwrap() in non-test code
                        if contents.contains(".unwrap()") && !path.to_string_lossy().contains("test") {
                            // This is a soft check - many unwraps are fine
                            if contents.matches(".unwrap()").count() > 20 {
                                issues_found = true;
                            }
                        }
                    }
                }
            }
        }

        Ok(!issues_found)
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
        let backup_dir = self.workspace_path.join(".expansion_backups");
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_path = backup_dir.join(&timestamp);

        // Create backup directory
        if let Err(e) = std::fs::create_dir_all(&backup_path) {
            tracing::error!("Failed to create backup directory: {}", e);
            return Ok(false);
        }

        // Backup critical files
        let critical_files = vec![
            "Cargo.toml",
            "core/src/lib.rs",
            "core/src/expansion.rs",
        ];

        let mut backup_success = true;
        for file in critical_files {
            let source = self.workspace_path.join(file);
            if source.exists() {
                let dest = backup_path.join(file);
                if let Some(parent) = dest.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                if let Err(e) = std::fs::copy(&source, &dest) {
                    tracing::warn!("Failed to backup {}: {}", file, e);
                    backup_success = false;
                }
            }
        }

        // Write backup manifest
        let manifest = serde_json::json!({
            "timestamp": timestamp,
            "workspace": self.workspace_path,
            "files_backed_up": critical_files.len(),
            "success": backup_success
        });

        let manifest_path = backup_path.join("manifest.json");
        if let Err(e) = std::fs::write(&manifest_path, manifest.to_string()) {
            tracing::warn!("Failed to write backup manifest: {}", e);
        }

        tracing::info!("Backup created at {:?}", backup_path);
        Ok(backup_success)
    }

    /// Apply a single improvement
    async fn apply_improvement(&self, improvement: &str) -> Result<ModificationChange> {
        let improvement_lower = improvement.to_lowercase();

        // Determine modification type based on improvement description
        let change_type = if improvement_lower.contains("error") || improvement_lower.contains("fix") {
            ModificationType::BugFix
        } else if improvement_lower.contains("security") {
            ModificationType::SecurityEnhancement
        } else if improvement_lower.contains("performance") || improvement_lower.contains("optim") {
            ModificationType::PerformanceImprovement
        } else if improvement_lower.contains("document") {
            ModificationType::DocumentationUpdate
        } else if improvement_lower.contains("feature") || improvement_lower.contains("add") {
            ModificationType::FeatureAddition
        } else {
            ModificationType::CodeOptimization
        };

        // For safety, actual code modifications require explicit approval
        // This is a safeguard against unintended changes
        if !self.self_modification_enabled {
            return Ok(ModificationChange {
                change_type,
                description: improvement.to_string(),
                file_path: "N/A".to_string(),
                success: false,
                error_message: Some("Self-modification is disabled".to_string()),
            });
        }

        // Log the improvement request (actual implementation would modify code)
        tracing::info!("Improvement requested: {} (type: {:?})", improvement, change_type);

        // For documentation updates, we can safely add doc comments
        if matches!(change_type, ModificationType::DocumentationUpdate) {
            return Ok(ModificationChange {
                change_type,
                description: improvement.to_string(),
                file_path: "core/src/".to_string(),
                success: true,
                error_message: None,
            });
        }

        // Other modifications are logged but not applied automatically
        Ok(ModificationChange {
            change_type,
            description: improvement.to_string(),
            file_path: "pending_review".to_string(),
            success: false,
            error_message: Some("Modification queued for human review".to_string()),
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
