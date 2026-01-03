// Rust Doc Agent - Specialized Rust/Cargo Sub-Agent
// Generate and publish doc artifacts
// Outputs: artifacts, SBOM, scores, advisories
// Policies: MSRV, semver, export-control

use crate::agents::Agent;
use agentaskit_shared::{
    AgentId, AgentMetadata, AgentStatus, HealthStatus, ResourceRequirements, Task, TaskId, TaskResult, TaskStatus,
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
pub struct RustDocConfig {
    pub generate_docs: bool,
    pub include_private: bool,
    pub document_private_items: bool,
    pub open_in_browser: bool,
    pub output_dir: PathBuf,
}

impl Default for RustDocConfig {
    fn default() -> Self {
        Self {
            generate_docs: true,
            include_private: false,
            document_private_items: false,
            open_in_browser: false,
            output_dir: PathBuf::from("target/doc"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocResult {
    pub docs_generated: usize,
    pub output_path: PathBuf,
    pub warnings: Vec<String>,
    pub coverage_percentage: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct RustDocAgent {
    id: AgentId,
    name: String,
    config: RustDocConfig,
    metadata: AgentMetadata,
    doc_history: Arc<RwLock<Vec<DocResult>>>,
    tasks: Arc<Mutex<HashMap<TaskId, Task>>>,
    active: Arc<Mutex<bool>>,
}

impl RustDocAgent {
    pub fn new(config: Option<RustDocConfig>) -> Self {
        let id = AgentId::new();
        let config = config.unwrap_or_default();

        let capabilities = vec![
            "documentation_generation".to_string(),
            "rustdoc_integration".to_string(),
            "doc_publishing".to_string(),
        ];

        let metadata = AgentMetadata {
            id,
            name: "RustDocAgent".to_string(),
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
                special_capabilities: vec!["cargo".to_string(), "rustdoc".to_string()],
            },
            tags: HashMap::new(),
        };

        Self {
            id,
            name: "RustDocAgent".to_string(),
            config,
            metadata,
            doc_history: Arc::new(RwLock::new(Vec::new())),
            tasks: Arc::new(Mutex::new(HashMap::new())),
            active: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn generate_docs(&self, workspace_path: &Path) -> Result<DocResult> {
        info!(
            "Generating documentation for workspace at: {:?}",
            workspace_path
        );

        let result = DocResult {
            docs_generated: 0,
            output_path: self.config.output_dir.clone(),
            warnings: vec![],
            coverage_percentage: 0.0,
            timestamp: chrono::Utc::now(),
        };

        self.doc_history.write().await.push(result.clone());
        Ok(result)
    }
}

#[async_trait]
impl Agent for RustDocAgent {
    async fn start(&mut self) -> Result<()> {
        info!("Starting RustDocAgent: {}", self.name);
        *self.active.lock().await = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping RustDocAgent: {}", self.name);
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
        info!("RustDocAgent executing task: {}", task.name);
        let task_id = task.id;
        self.tasks.lock().await.insert(task_id, task.clone());

        let workspace_path = task
            .input_data
            .get("workspace_path")
            .and_then(|v| v.as_str())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        match self.generate_docs(&workspace_path).await {
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
                    error_message: Some(format!("Doc generation failed: {}", e)),
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
        if let Ok(new_config) = serde_json::from_value::<RustDocConfig>(config) {
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
