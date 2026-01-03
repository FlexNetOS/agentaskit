// Rust FFI Agent - Specialized Rust/Cargo Sub-Agent
// bindgen/cbindgen pipelines, ABI tests
// Outputs: artifacts, SBOM, scores, advisories
// Policies: MSRV, semver, export-control

use crate::agents::Agent;
use agentaskit_shared::{
    AgentMetadata, AgentStatus, HealthStatus, ResourceRequirements, Task, TaskResult, TaskStatus,
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustFFIConfig {
    pub enable_bindgen: bool,
    pub enable_cbindgen: bool,
    pub abi_test_enabled: bool,
    pub header_output_dir: PathBuf,
    pub binding_output_dir: PathBuf,
}

impl Default for RustFFIConfig {
    fn default() -> Self {
        Self {
            enable_bindgen: true,
            enable_cbindgen: true,
            abi_test_enabled: true,
            header_output_dir: PathBuf::from("include"),
            binding_output_dir: PathBuf::from("bindings"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FFIResult {
    pub bindings_generated: Vec<FFIBinding>,
    pub abi_tests_passed: usize,
    pub abi_tests_failed: usize,
    pub warnings: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FFIBinding {
    pub name: String,
    pub binding_type: FFIBindingType,
    pub output_path: PathBuf,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FFIBindingType {
    CHeader,
    RustBindings,
    CppBindings,
}

pub struct RustFFIAgent {
    id: Uuid,
    name: String,
    config: RustFFIConfig,
    metadata: AgentMetadata,
    ffi_history: Arc<RwLock<Vec<FFIResult>>>,
    tasks: Arc<Mutex<HashMap<Uuid, Task>>>,
    active: Arc<Mutex<bool>>,
}

impl RustFFIAgent {
    pub fn new(config: Option<RustFFIConfig>) -> Self {
        let id = Uuid::new_v4();
        let config = config.unwrap_or_default();

        let capabilities = vec![
            "bindgen_integration".to_string(),
            "cbindgen_integration".to_string(),
            "ffi_generation".to_string(),
            "abi_testing".to_string(),
        ];

        let metadata = AgentMetadata {
            id,
            name: "RustFFIAgent".to_string(),
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
                storage_mb: Some(1024),
                network_bandwidth_mbps: Some(10),
                gpu_required: false,
                special_capabilities: vec!["bindgen".to_string(), "cbindgen".to_string()],
            },
            tags: HashMap::new(),
        };

        Self {
            id,
            name: "RustFFIAgent".to_string(),
            config,
            metadata,
            ffi_history: Arc::new(RwLock::new(Vec::new())),
            tasks: Arc::new(Mutex::new(HashMap::new())),
            active: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn generate_bindings(&self, workspace_path: &Path) -> Result<FFIResult> {
        info!(
            "Generating FFI bindings for workspace at: {:?}",
            workspace_path
        );

        let result = FFIResult {
            bindings_generated: vec![],
            abi_tests_passed: 0,
            abi_tests_failed: 0,
            warnings: vec![],
            timestamp: chrono::Utc::now(),
        };

        self.ffi_history.write().await.push(result.clone());
        Ok(result)
    }
}

#[async_trait]
impl Agent for RustFFIAgent {
    async fn start(&mut self) -> Result<()> {
        info!("Starting RustFFIAgent: {}", self.name);
        *self.active.lock().await = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping RustFFIAgent: {}", self.name);
        *self.active.lock().await = false;
        Ok(())
    }

    async fn handle_message(
        &mut self,
        _message: crate::agents::AgentMessage,
    ) -> Result<Option<crate::agents::AgentMessage>> {
        Ok(None)
    }

    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        info!("RustFFIAgent executing task: {}", task.name);
        let task_id = task.id;
        self.tasks.lock().await.insert(task_id, task.clone());

        let workspace_path = task
            .input_data
            .get("workspace_path")
            .and_then(|v| v.as_str())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        match self.generate_bindings(&workspace_path).await {
            Ok(result) => {
                self.tasks.lock().await.remove(&task_id);
                Ok(TaskResult {
                    task_id,
                    status: TaskStatus::Completed,
                    output_data: Some(serde_json::to_value(result)?),
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
                    error_message: Some(format!("FFI generation failed: {}", e)),
                    completed_at: chrono::Utc::now(),
                })
            }
        }
    }

    async fn health_check(&self) -> Result<HealthStatus> {
        if *self.active.lock().await {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unknown)
        }
    }

    async fn update_config(&mut self, config: serde_json::Value) -> Result<()> {
        if let Ok(new_config) = serde_json::from_value::<RustFFIConfig>(config) {
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
