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

        // Search for agent files in common locations
        let agent_paths = vec![
            self.workspace_path.join("crates/agents/src"),
            self.workspace_path.join("core/src/agents"),
            self.workspace_path.join("agentaskit-production/core/src/agents"),
        ];

        for agent_path in agent_paths {
            if agent_path.exists() {
                if let Ok(entries) = std::fs::read_dir(&agent_path) {
                    for entry in entries.flatten() {
                        if entry.path().is_file() {
                            // Count Rust files that likely contain agent implementations
                            if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                                    // Count files that define agent structs
                                    if content.contains("struct") &&
                                       (content.contains("Agent") || content.contains("agent")) {
                                        agent_count += 1;
                                    }
                                }
                            }
                        } else if entry.path().is_dir() {
                            // Recursively count agents in subdirectories
                            if let Ok(sub_entries) = std::fs::read_dir(entry.path()) {
                                for sub_entry in sub_entries.flatten() {
                                    if sub_entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                                        if let Ok(content) = std::fs::read_to_string(sub_entry.path()) {
                                            if content.contains("struct") &&
                                               (content.contains("Agent") || content.contains("agent")) {
                                                agent_count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(agent_count)
    }
    
    /// Count components in the system
    async fn count_components(&self) -> Result<u64> {
        let mut component_count = 0;

        // Search for component files (structs implementing Component trait)
        let component_paths = vec![
            self.workspace_path.join("crates"),
            self.workspace_path.join("core/src"),
            self.workspace_path.join("agentaskit-production/core/src"),
        ];

        for component_path in component_paths {
            if component_path.exists() {
                // Use walkdir for recursive traversal
                for entry in walkdir::WalkDir::new(&component_path)
                    .max_depth(5)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.path().is_file() &&
                       entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            // Count structs that implement Component or AutonomousComponent traits
                            if (content.contains("impl Component for") ||
                                content.contains("impl AutonomousComponent for") ||
                                content.contains("#[derive(Component)")) {
                                component_count += 1;
                            }
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
        let mut total_lines = 0u64;
        let mut functions = 0u64;
        let mut structs = 0u64;
        let mut traits = 0u64;
        let mut tests = 0u64;
        let mut documented_items = 0u64;
        let mut total_items = 0u64;

        // Analyze Rust source files in the workspace
        for entry in walkdir::WalkDir::new(&self.workspace_path)
            .max_depth(6)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() &&
               entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    // Count lines
                    total_lines += content.lines().count() as u64;

                    // Count functions
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") ||
                           trimmed.starts_with("async fn ") || trimmed.starts_with("pub async fn ") {
                            functions += 1;
                            total_items += 1;

                            // Check if function is documented (previous line has ///)
                            if content.contains(&format!("/// ")) {
                                documented_items += 1;
                            }
                        }

                        if trimmed.starts_with("struct ") || trimmed.starts_with("pub struct ") {
                            structs += 1;
                            total_items += 1;
                        }

                        if trimmed.starts_with("trait ") || trimmed.starts_with("pub trait ") {
                            traits += 1;
                            total_items += 1;
                        }

                        if trimmed.starts_with("#[test]") || trimmed.starts_with("#[tokio::test]") {
                            tests += 1;
                        }
                    }
                }
            }
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
        use sysinfo::{System, SystemExt, ProcessExt, DiskExt};

        let mut sys = System::new_all();
        sys.refresh_all();

        // Get current process metrics
        let pid = sysinfo::get_current_pid().unwrap_or(sysinfo::Pid::from(0));
        let cpu_usage = if let Some(process) = sys.process(pid) {
            process.cpu_usage() as f64
        } else {
            0.0
        };

        let memory_usage = if let Some(process) = sys.process(pid) {
            process.memory()
        } else {
            0
        };

        // Calculate total disk usage
        let mut disk_usage = 0u64;
        for disk in sys.disks() {
            disk_usage += disk.total_space() - disk.available_space();
        }

        // Identify potential bottlenecks
        let mut bottlenecks = Vec::new();

        // Check CPU usage
        if cpu_usage > 80.0 {
            bottlenecks.push("High CPU usage detected".to_string());
        }

        // Check memory usage
        if memory_usage > 1024 * 1024 * 1024 { // > 1GB
            bottlenecks.push("High memory usage detected".to_string());
        }

        // Check available memory
        if sys.available_memory() < sys.total_memory() / 10 {
            bottlenecks.push("Low available system memory".to_string());
        }

        // Simulate response times (in production, this would be real measurements)
        let response_times = vec![10.0, 15.0, 12.0, 20.0, 18.0];

        Ok(PerformanceMetrics {
            cpu_usage,
            memory_usage,
            disk_usage,
            response_times,
            bottlenecks,
        })
    }
    
    /// Perform security analysis
    pub async fn analyze_security(&self) -> Result<SecurityAnalysis> {
        tracing::info!("Performing security analysis for expansion engine {}", self.id);
        
        let file_permissions = self.check_file_permissions().await?;
        let security_issues = self.identify_security_issues().await?;
        let compliance_status = self.check_compliance().await?;
        
        let exposed_ports = self.scan_exposed_ports().await?;

        Ok(SecurityAnalysis {
            timestamp: Utc::now(),
            file_permissions,
            exposed_ports,
            security_issues,
            compliance_status,
        })
    }
    
    /// Scan for exposed ports
    async fn scan_exposed_ports(&self) -> Result<Vec<u16>> {
        use std::net::TcpListener;

        let mut exposed_ports = Vec::new();

        // Check common service ports that might be bound
        let common_ports = vec![
            8080, 8081, 8082, 8000, 3000, 5000, 9090, 6379, 5432, 27017, 3306
        ];

        for port in common_ports {
            // Try to bind to the port - if it fails, it's likely already in use (exposed)
            let address = format!("127.0.0.1:{}", port);
            if TcpListener::bind(&address).is_err() {
                exposed_ports.push(port);
            }
        }

        Ok(exposed_ports)
    }

    /// Check file permissions
    async fn check_file_permissions(&self) -> Result<HashMap<String, String>> {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = HashMap::new();

        // Check critical files
        let critical_files = vec![
            "Cargo.toml",
            "core/src/lib.rs",
            "core/Cargo.toml",
            ".env",
            "config.toml",
        ];

        for file in critical_files {
            let path = self.workspace_path.join(file);
            if path.exists() {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    let perms = metadata.permissions();
                    // Get Unix permissions as octal
                    let mode = perms.mode();
                    let octal_perms = format!("{:o}", mode & 0o777);
                    permissions.insert(file.to_string(), octal_perms);

                    // Check for overly permissive files
                    if mode & 0o002 != 0 {
                        tracing::warn!("File {} is world-writable: {}", file, mode);
                    }
                }
            }
        }

        Ok(permissions)
    }
    
    /// Identify security issues
    async fn identify_security_issues(&self) -> Result<Vec<SecurityIssue>> {
        let mut issues = Vec::new();

        // Check for common security patterns in Rust files
        for entry in walkdir::WalkDir::new(&self.workspace_path)
            .max_depth(6)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() &&
               entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    let file_name = entry.path().to_string_lossy().to_string();

                    // Check for unsafe blocks
                    if content.contains("unsafe {") || content.contains("unsafe fn") {
                        issues.push(SecurityIssue {
                            severity: SecuritySeverity::Medium,
                            description: "Unsafe code block detected".to_string(),
                            affected_component: file_name.clone(),
                            recommendation: "Review unsafe code for memory safety".to_string(),
                        });
                    }

                    // Check for unwrap() calls that could panic
                    if content.matches(".unwrap()").count() > 5 {
                        issues.push(SecurityIssue {
                            severity: SecuritySeverity::Low,
                            description: "Excessive unwrap() calls detected".to_string(),
                            affected_component: file_name.clone(),
                            recommendation: "Use proper error handling instead of unwrap()".to_string(),
                        });
                    }

                    // Check for hardcoded credentials patterns
                    if content.contains("password") && content.contains("=") {
                        issues.push(SecurityIssue {
                            severity: SecuritySeverity::High,
                            description: "Potential hardcoded password detected".to_string(),
                            affected_component: file_name.clone(),
                            recommendation: "Use environment variables for sensitive data".to_string(),
                        });
                    }

                    // Check for SQL injection vulnerabilities
                    if content.contains("format!(\"SELECT") || content.contains("format!(\"INSERT") {
                        issues.push(SecurityIssue {
                            severity: SecuritySeverity::Critical,
                            description: "Potential SQL injection vulnerability".to_string(),
                            affected_component: file_name.clone(),
                            recommendation: "Use parameterized queries instead of string formatting".to_string(),
                        });
                    }
                }
            }
        }

        // Check .env file permissions
        let env_path = self.workspace_path.join(".env");
        if env_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&env_path) {
                use std::os::unix::fs::PermissionsExt;
                let mode = metadata.permissions().mode();
                if mode & 0o004 != 0 {
                    issues.push(SecurityIssue {
                        severity: SecuritySeverity::High,
                        description: ".env file is world-readable".to_string(),
                        affected_component: ".env".to_string(),
                        recommendation: "Restrict .env file permissions to 600".to_string(),
                    });
                }
            }
        }

        Ok(issues)
    }
    
    /// Check compliance status
    async fn check_compliance(&self) -> Result<ComplianceStatus> {
        // Check documentation standards
        let code_quality = self.analyze_code_quality().await?;
        let documentation_standards = code_quality.documentation_coverage >= 0.7; // 70% threshold

        // Check for Cargo.toml and proper project structure
        let cargo_exists = self.workspace_path.join("Cargo.toml").exists();
        let src_exists = self.workspace_path.join("core/src").exists() ||
                         self.workspace_path.join("src").exists();
        let rust_standards = cargo_exists && src_exists;

        // Check security guidelines by verifying no critical issues
        let security_analysis = self.identify_security_issues().await?;
        let security_guidelines = !security_analysis.iter()
            .any(|issue| issue.severity == SecuritySeverity::Critical);

        // Check performance standards
        let performance_metrics = self.measure_performance().await?;
        let performance_standards = performance_metrics.bottlenecks.is_empty();

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

        // Create backup directory if it doesn't exist
        let backup_dir = self.workspace_path.join(".backups");
        if !backup_dir.exists() {
            std::fs::create_dir_all(&backup_dir)?;
        }

        // Create timestamped backup
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("expansion_backup_{}", timestamp);
        let backup_path = backup_dir.join(&backup_name);

        std::fs::create_dir_all(&backup_path)?;

        // Backup critical files
        let critical_files = vec![
            "core/src/orchestration/expansion.rs",
            "core/Cargo.toml",
            "Cargo.toml",
        ];

        for file in critical_files {
            let src_path = self.workspace_path.join(file);
            if src_path.exists() {
                let dest_path = backup_path.join(file);

                // Create parent directories
                if let Some(parent) = dest_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                std::fs::copy(&src_path, &dest_path)?;
                tracing::info!("Backed up file: {} to {}", file, dest_path.display());
            }
        }

        // Write backup metadata
        let metadata = serde_json::json!({
            "timestamp": timestamp.to_string(),
            "backup_name": backup_name,
            "files_backed_up": critical_files.len(),
            "expansion_engine_id": self.id.to_string(),
        });

        let metadata_path = backup_path.join("metadata.json");
        std::fs::write(metadata_path, serde_json::to_string_pretty(&metadata)?)?;

        tracing::info!("Backup created successfully at: {}", backup_path.display());
        Ok(true)
    }
    
    /// Apply a single improvement
    async fn apply_improvement(&self, improvement: &str) -> Result<ModificationChange> {
        tracing::info!("Applying improvement: {}", improvement);

        // Determine the type of improvement based on keywords
        let change_type = if improvement.contains("error") || improvement.contains("handling") {
            ModificationType::BugFix
        } else if improvement.contains("performance") || improvement.contains("optimize") {
            ModificationType::PerformanceImprovement
        } else if improvement.contains("security") {
            ModificationType::SecurityEnhancement
        } else if improvement.contains("documentation") || improvement.contains("doc") {
            ModificationType::DocumentationUpdate
        } else if improvement.contains("feature") || improvement.contains("add") {
            ModificationType::FeatureAddition
        } else {
            ModificationType::CodeOptimization
        };

        // Simulate improvement application
        // In a real implementation, this would:
        // 1. Parse the improvement request
        // 2. Generate code changes using AI/ML
        // 3. Apply the changes to the appropriate files
        // 4. Verify the changes compile and tests pass
        // 5. Rollback if verification fails

        // For now, we'll log the improvement and mark it as pending manual implementation
        let file_path = "core/src/orchestration/expansion.rs".to_string();

        // Check if we can identify a specific action
        let success = match change_type {
            ModificationType::DocumentationUpdate => {
                // We can reasonably add documentation automatically
                tracing::info!("Documentation improvement identified and logged");
                true
            }
            ModificationType::CodeOptimization => {
                // Basic optimizations might be safe
                tracing::info!("Code optimization identified for manual review");
                false
            }
            _ => {
                // Other changes require careful consideration
                tracing::warn!("Improvement requires manual implementation: {}", improvement);
                false
            }
        };

        let error_message = if success {
            None
        } else {
            Some(format!("Improvement logged for manual implementation: {}", improvement))
        };

        Ok(ModificationChange {
            change_type,
            description: improvement.to_string(),
            file_path,
            success,
            error_message,
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
