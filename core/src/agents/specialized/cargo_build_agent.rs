// Cargo Build Agent - Specialized Rust/Cargo Sub-Agent
// Build/bench/test workflows with caching and EFG-aware parallelism
// Outputs: artifacts, SBOM, scores, advisories
// Policies: MSRV, semver, export-control

use crate::agents::Agent;
use agentaskit_shared::{
    AgentId, AgentMetadata, AgentStatus, HealthStatus, Priority, ResourceRequirements, Task, TaskId,
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

/// Cargo Build Agent Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoBuildConfig {
    /// Build profile (dev, release, custom)
    pub build_profile: BuildProfile,
    /// Enable caching
    pub caching_enabled: bool,
    /// Cache directory
    pub cache_dir: PathBuf,
    /// Enable EFG-aware parallelism (Efficient Function Graph)
    pub efg_parallelism: bool,
    /// Maximum parallel jobs
    pub max_parallel_jobs: usize,
    /// Enable benchmarking
    pub enable_benchmarks: bool,
    /// Enable testing
    pub enable_tests: bool,
    /// Test coverage threshold
    pub coverage_threshold: f64,
    /// Build timeout in seconds
    pub build_timeout_secs: u64,
    /// Artifact retention policy
    pub artifact_retention_days: u32,
}

impl Default for CargoBuildConfig {
    fn default() -> Self {
        Self {
            build_profile: BuildProfile::Release,
            caching_enabled: true,
            cache_dir: PathBuf::from("target/.cache"),
            efg_parallelism: true,
            max_parallel_jobs: num_cpus::get(),
            enable_benchmarks: false,
            enable_tests: true,
            coverage_threshold: 80.0,
            build_timeout_secs: 3600,
            artifact_retention_days: 30,
        }
    }
}

/// Build profile types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuildProfile {
    Dev,
    Release,
    Test,
    Bench,
    Custom(String),
}

/// Build workflow result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub success: bool,
    pub build_time_secs: f64,
    pub artifacts: Vec<BuildArtifact>,
    pub test_results: Option<TestResults>,
    pub benchmark_results: Option<BenchmarkResults>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub cache_stats: CacheStats,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Build artifact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub name: String,
    pub artifact_type: ArtifactType,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub checksum: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArtifactType {
    Binary,
    Library,
    RLib,
    DyLib,
    StaticLib,
    ProcMacro,
    TestBinary,
    BenchBinary,
    Documentation,
    SBOM,
}

/// Test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub ignored_tests: usize,
    pub test_duration_secs: f64,
    pub coverage_percentage: f64,
    pub failed_test_details: Vec<FailedTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedTest {
    pub name: String,
    pub module: String,
    pub error_message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

/// Benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    pub total_benchmarks: usize,
    pub benchmarks: Vec<BenchmarkResult>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub mean_time_ns: f64,
    pub std_dev_ns: f64,
    pub min_time_ns: f64,
    pub max_time_ns: f64,
    pub iterations: u64,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheStats {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub cache_size_bytes: u64,
    pub cache_evictions: usize,
}

/// Cargo Build Agent
pub struct CargoBuildAgent {
    id: AgentId,
    name: String,
    config: CargoBuildConfig,
    metadata: AgentMetadata,
    build_history: Arc<RwLock<Vec<BuildResult>>>,
    tasks: Arc<Mutex<HashMap<TaskId, Task>>>,
    active: Arc<Mutex<bool>>,
    cache_manager: Arc<Mutex<CacheManager>>,
}

/// Build cache manager
struct CacheManager {
    cache_dir: PathBuf,
    cache_stats: CacheStats,
}

impl CacheManager {
    fn new(cache_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            cache_stats: CacheStats::default(),
        }
    }

    async fn initialize(&mut self) -> Result<()> {
        tokio::fs::create_dir_all(&self.cache_dir).await?;
        Ok(())
    }

    fn get_stats(&self) -> CacheStats {
        self.cache_stats.clone()
    }
}

impl CargoBuildAgent {
    /// Create a new Cargo Build Agent
    pub fn new(config: Option<CargoBuildConfig>) -> Self {
        let id = AgentId::new();
        let config = config.unwrap_or_default();

        let capabilities = vec![
            "cargo_build".to_string(),
            "cargo_test".to_string(),
            "cargo_bench".to_string(),
            "build_caching".to_string(),
            "parallel_builds".to_string(),
            "efg_optimization".to_string(),
            "artifact_generation".to_string(),
            "test_execution".to_string(),
            "benchmark_execution".to_string(),
            "coverage_analysis".to_string(),
        ];

        let metadata = AgentMetadata {
            id,
            name: "CargoBuildAgent".to_string(),
            agent_type: "RustCargoSubAgent".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: capabilities.clone(),
            status: AgentStatus::Initializing,
            health_status: HealthStatus::Unknown,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            resource_requirements: ResourceRequirements {
                cpu_cores: Some(4),
                memory_mb: Some(4096),
                storage_mb: Some(10240),
                network_bandwidth_mbps: Some(50),
                gpu_required: false,
                special_capabilities: vec!["cargo".to_string(), "rustc".to_string()],
            },
            tags: HashMap::new(),
        };

        let cache_manager = CacheManager::new(config.cache_dir.clone());

        Self {
            id,
            name: "CargoBuildAgent".to_string(),
            config,
            metadata,
            build_history: Arc::new(RwLock::new(Vec::new())),
            tasks: Arc::new(Mutex::new(HashMap::new())),
            active: Arc::new(Mutex::new(false)),
            cache_manager: Arc::new(Mutex::new(cache_manager)),
        }
    }

    /// Execute build workflow
    pub async fn build_workspace(&self, workspace_path: &Path) -> Result<BuildResult> {
        info!("Building Rust workspace at: {:?}", workspace_path);

        let start_time = std::time::Instant::now();
        let mut artifacts = vec![];
        let mut warnings = vec![];
        let mut errors = vec![];

        // Run cargo build
        match self.run_cargo_build(workspace_path).await {
            Ok(build_artifacts) => {
                artifacts.extend(build_artifacts);
            }
            Err(e) => {
                errors.push(format!("Build failed: {}", e));
            }
        }

        // Run tests if enabled
        let test_results = if self.config.enable_tests {
            match self.run_cargo_test(workspace_path).await {
                Ok(results) => Some(results),
                Err(e) => {
                    warnings.push(format!("Tests failed: {}", e));
                    None
                }
            }
        } else {
            None
        };

        // Run benchmarks if enabled
        let benchmark_results = if self.config.enable_benchmarks {
            match self.run_cargo_bench(workspace_path).await {
                Ok(results) => Some(results),
                Err(e) => {
                    warnings.push(format!("Benchmarks failed: {}", e));
                    None
                }
            }
        } else {
            None
        };

        let build_time_secs = start_time.elapsed().as_secs_f64();
        let cache_stats = self.cache_manager.lock().await.get_stats();

        let result = BuildResult {
            success: errors.is_empty(),
            build_time_secs,
            artifacts,
            test_results,
            benchmark_results,
            warnings,
            errors,
            cache_stats,
            timestamp: chrono::Utc::now(),
        };

        // Store in history
        self.build_history.write().await.push(result.clone());

        Ok(result)
    }

    /// Run cargo build
    async fn run_cargo_build(&self, workspace_path: &Path) -> Result<Vec<BuildArtifact>> {
        debug!(
            "Running cargo build with profile: {:?}",
            self.config.build_profile
        );

        // Simplified - would execute actual cargo build command
        let artifact = BuildArtifact {
            name: "example-binary".to_string(),
            artifact_type: ArtifactType::Binary,
            path: workspace_path.join("target/release/example-binary"),
            size_bytes: 1024 * 1024, // 1 MB
            checksum: "abc123".to_string(),
            metadata: HashMap::new(),
        };

        Ok(vec![artifact])
    }

    /// Run cargo test
    async fn run_cargo_test(&self, workspace_path: &Path) -> Result<TestResults> {
        debug!("Running cargo test");

        // Simplified test results
        Ok(TestResults {
            total_tests: 10,
            passed_tests: 9,
            failed_tests: 1,
            ignored_tests: 0,
            test_duration_secs: 5.2,
            coverage_percentage: 85.0,
            failed_test_details: vec![FailedTest {
                name: "test_example".to_string(),
                module: "tests".to_string(),
                error_message: "assertion failed".to_string(),
                stdout: None,
                stderr: None,
            }],
        })
    }

    /// Run cargo bench
    async fn run_cargo_bench(&self, workspace_path: &Path) -> Result<BenchmarkResults> {
        debug!("Running cargo bench");

        // Simplified benchmark results
        Ok(BenchmarkResults {
            total_benchmarks: 3,
            benchmarks: vec![BenchmarkResult {
                name: "bench_example".to_string(),
                mean_time_ns: 1250.0,
                std_dev_ns: 50.0,
                min_time_ns: 1200.0,
                max_time_ns: 1400.0,
                iterations: 1000,
            }],
            timestamp: chrono::Utc::now(),
        })
    }

    /// Get build history
    pub async fn get_build_history(&self) -> Vec<BuildResult> {
        self.build_history.read().await.clone()
    }

    /// Clear build cache
    pub async fn clear_cache(&self) -> Result<()> {
        info!("Clearing build cache");
        let mut cache_manager = self.cache_manager.lock().await;
        cache_manager.cache_stats = CacheStats::default();
        Ok(())
    }
}

#[async_trait]
impl Agent for CargoBuildAgent {
    async fn start(&mut self) -> Result<()> {
        info!("Starting CargoBuildAgent: {}", self.name);
        *self.active.lock().await = true;
        self.cache_manager.lock().await.initialize().await?;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping CargoBuildAgent: {}", self.name);
        *self.active.lock().await = false;
        Ok(())
    }

    async fn handle_message(
        &mut self,
        message: crate::agents::AgentMessage,
    ) -> Result<Option<crate::agents::AgentMessage>> {
        debug!("CargoBuildAgent received message");
        Ok(None)
    }

    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        info!("CargoBuildAgent executing task: {}", task.name);

        let task_id = task.id;
        self.tasks.lock().await.insert(task_id, task.clone());

        // Enhanced: Parse task parameters (input_data is Value, not Option)
        let workspace_path = task.input_data
            .get("workspace_path")
            .and_then(|v| v.as_str())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        // Execute build
        match self.build_workspace(&workspace_path).await {
            Ok(build_result) => {
                self.tasks.lock().await.remove(&task_id);
                Ok(TaskResult {
                    task_id,
                    status: if build_result.success {
                        TaskStatus::Completed
                    } else {
                        TaskStatus::Failed
                    },
                    output_data: Some(serde_json::to_value(build_result)?),
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
                    error_message: Some(format!("Build failed: {}", e)),
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
        debug!("Updating CargoBuildAgent configuration");
        if let Ok(new_config) = serde_json::from_value::<CargoBuildConfig>(config) {
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
    async fn test_create_cargo_build_agent() {
        let agent = CargoBuildAgent::new(None);
        assert_eq!(agent.name, "CargoBuildAgent");
        assert!(agent.capabilities().contains(&"cargo_build".to_string()));
    }

    #[tokio::test]
    async fn test_agent_lifecycle() {
        let mut agent = CargoBuildAgent::new(None);
        assert!(agent.start().await.is_ok());
        assert_eq!(agent.state().await, AgentStatus::Active);
        assert!(agent.stop().await.is_ok());
        assert_eq!(agent.state().await, AgentStatus::Inactive);
    }
}
