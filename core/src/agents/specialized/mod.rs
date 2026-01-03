// Specialized Layer - Phase 4 Micro-Agent Framework
// Domain expert agents providing operational capabilities with autonomous execution

pub mod code_generation_agent;
pub mod data_analytics_agent;
pub mod deployment_agent;
pub mod integration_agent;
pub mod learning_agent;
pub mod monitoring_agent;
pub mod security_specialist_agent;
pub mod testing_agent;

// Rust/Cargo Sub-Agents
pub mod cargo_audit_agent;
pub mod cargo_build_agent;
pub mod cargo_license_agent;
pub mod rust_clippy_agent;
pub mod rust_crate_scanner_agent;
pub mod rust_cross_agent;
pub mod rust_doc_agent;
pub mod rust_ffi_agent;
pub mod rust_fmt_agent;
pub mod rust_release_agent;
pub mod rust_wasm_agent;

// Re-export all specialized agents for easy access
pub use code_generation_agent::CodeGenerationAgent;
pub use data_analytics_agent::DataAnalyticsAgent;
pub use deployment_agent::DeploymentAgent;
pub use integration_agent::IntegrationAgent;
pub use learning_agent::LearningAgent;
pub use monitoring_agent::MonitoringAgent;
pub use security_specialist_agent::SecuritySpecialistAgent;
pub use testing_agent::TestingAgent;

// Re-export Rust/Cargo sub-agents
pub use cargo_audit_agent::CargoAuditAgent;
pub use cargo_build_agent::CargoBuildAgent;
pub use cargo_license_agent::CargoLicenseAgent;
pub use rust_clippy_agent::RustClippyAgent;
pub use rust_crate_scanner_agent::RustCrateScannerAgent;
pub use rust_cross_agent::RustCrossAgent;
pub use rust_doc_agent::RustDocAgent;
pub use rust_ffi_agent::RustFFIAgent;
pub use rust_fmt_agent::RustFmtAgent;
pub use rust_release_agent::RustReleaseAgent;
pub use rust_wasm_agent::RustWasmAgent;

use crate::agents::{Agent, AgentMessage};
use crate::orchestration::Task;
use agentaskit_shared::{AgentId, AgentStatus, TaskResult};
use anyhow::Result as AgentResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

/// Enhanced: Specialized Layer Coordinator with type-safe IDs
/// Manages the collection of domain expert agents and their interactions
pub struct SpecializedLayer {
    agents: Arc<RwLock<HashMap<AgentId, Box<dyn Agent>>>>,
    agent_registry: Arc<RwLock<HashMap<String, AgentId>>>,
}

impl SpecializedLayer {
    /// Create new Specialized Layer with all domain expert agents
    pub async fn new() -> AgentResult<Self> {
        let layer = Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            agent_registry: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize all specialized agents
        layer.initialize_agents().await?;

        Ok(layer)
    }

    /// Initialize all specialized agents with their default configurations
    async fn initialize_agents(&self) -> AgentResult<()> {
        info!("Initializing Specialized Layer with all domain expert agents");

        let mut agents = self.agents.write().await;
        let mut registry = self.agent_registry.write().await;

        // Code Generation Agent
        let code_gen_agent = Box::new(CodeGenerationAgent::new(None));
        let code_gen_id = code_gen_agent.metadata().id;
        registry.insert("code_generation".to_string(), code_gen_id);
        agents.insert(code_gen_id, code_gen_agent);

        // Testing Agent
        let testing_agent = Box::new(TestingAgent::new(None));
        let testing_id = testing_agent.metadata().id;
        registry.insert("testing".to_string(), testing_id);
        agents.insert(testing_id, testing_agent);

        // Deployment Agent
        let deployment_agent = Box::new(DeploymentAgent::new(None));
        let deployment_id = deployment_agent.metadata().id;
        registry.insert("deployment".to_string(), deployment_id);
        agents.insert(deployment_id, deployment_agent);

        // Monitoring Agent
        let monitoring_agent = Box::new(MonitoringAgent::new(None));
        let monitoring_id = monitoring_agent.metadata().id;
        registry.insert("monitoring".to_string(), monitoring_id);
        agents.insert(monitoring_id, monitoring_agent);

        // Learning Agent
        let learning_agent = Box::new(LearningAgent::new(None));
        let learning_id = learning_agent.metadata().id;
        registry.insert("learning".to_string(), learning_id);
        agents.insert(learning_id, learning_agent);

        // Security Specialist Agent
        let security_agent = Box::new(SecuritySpecialistAgent::new(None));
        let security_id = security_agent.metadata().id;
        registry.insert("security_specialist".to_string(), security_id);
        agents.insert(security_id, security_agent);

        // Data Analytics Agent
        let analytics_agent = Box::new(DataAnalyticsAgent::new(None));
        let analytics_id = analytics_agent.metadata().id;
        registry.insert("data_analytics".to_string(), analytics_id);
        agents.insert(analytics_id, analytics_agent);

        // Integration Agent
        let integration_agent = Box::new(IntegrationAgent::new(None));
        let integration_id = integration_agent.metadata().id;
        registry.insert("integration".to_string(), integration_id);
        agents.insert(integration_id, integration_agent);

        // Rust/Cargo Sub-Agents

        // Rust Crate Scanner Agent
        let rust_crate_scanner = Box::new(RustCrateScannerAgent::new(None));
        let rust_crate_scanner_id = rust_crate_scanner.metadata().id;
        registry.insert("rust_crate_scanner".to_string(), rust_crate_scanner_id);
        agents.insert(rust_crate_scanner_id, rust_crate_scanner);

        // Cargo Build Agent
        let cargo_build = Box::new(CargoBuildAgent::new(None));
        let cargo_build_id = cargo_build.metadata().id;
        registry.insert("cargo_build".to_string(), cargo_build_id);
        agents.insert(cargo_build_id, cargo_build);

        // Cargo Audit Agent
        let cargo_audit = Box::new(CargoAuditAgent::new(None));
        let cargo_audit_id = cargo_audit.metadata().id;
        registry.insert("cargo_audit".to_string(), cargo_audit_id);
        agents.insert(cargo_audit_id, cargo_audit);

        // Cargo License Agent
        let cargo_license = Box::new(CargoLicenseAgent::new(None));
        let cargo_license_id = cargo_license.metadata().id;
        registry.insert("cargo_license".to_string(), cargo_license_id);
        agents.insert(cargo_license_id, cargo_license);

        // Rust Clippy Agent
        let rust_clippy = Box::new(RustClippyAgent::new(None));
        let rust_clippy_id = rust_clippy.metadata().id;
        registry.insert("rust_clippy".to_string(), rust_clippy_id);
        agents.insert(rust_clippy_id, rust_clippy);

        // Rust Fmt Agent
        let rust_fmt = Box::new(RustFmtAgent::new(None));
        let rust_fmt_id = rust_fmt.metadata().id;
        registry.insert("rust_fmt".to_string(), rust_fmt_id);
        agents.insert(rust_fmt_id, rust_fmt);

        // Rust Doc Agent
        let rust_doc = Box::new(RustDocAgent::new(None));
        let rust_doc_id = rust_doc.metadata().id;
        registry.insert("rust_doc".to_string(), rust_doc_id);
        agents.insert(rust_doc_id, rust_doc);

        // Rust FFI Agent
        let rust_ffi = Box::new(RustFFIAgent::new(None));
        let rust_ffi_id = rust_ffi.metadata().id;
        registry.insert("rust_ffi".to_string(), rust_ffi_id);
        agents.insert(rust_ffi_id, rust_ffi);

        // Rust WASM Agent
        let rust_wasm = Box::new(RustWasmAgent::new(None));
        let rust_wasm_id = rust_wasm.metadata().id;
        registry.insert("rust_wasm".to_string(), rust_wasm_id);
        agents.insert(rust_wasm_id, rust_wasm);

        // Rust Cross Agent
        let rust_cross = Box::new(RustCrossAgent::new(None));
        let rust_cross_id = rust_cross.metadata().id;
        registry.insert("rust_cross".to_string(), rust_cross_id);
        agents.insert(rust_cross_id, rust_cross);

        // Rust Release Agent
        let rust_release = Box::new(RustReleaseAgent::new(None));
        let rust_release_id = rust_release.metadata().id;
        registry.insert("rust_release".to_string(), rust_release_id);
        agents.insert(rust_release_id, rust_release);

        info!(
            "Specialized Layer initialized with {} domain expert agents (including {} Rust/Cargo sub-agents)",
            agents.len(),
            11
        );
        Ok(())
    }

    /// Start all specialized agents
    pub async fn start_all_agents(&self) -> AgentResult<()> {
        info!("Starting all Specialized Layer agents");

        // Note: Agents are started individually when needed
        // The layer doesn't manage the lifecycle of individual agents
        info!("Specialized Layer agents are ready to be started individually");
        Ok(())
    }

    /// Stop all specialized agents
    pub async fn stop_all_agents(&self) -> AgentResult<()> {
        info!("Stopping all Specialized Layer agents");

        // Note: Agents should be stopped individually
        info!("Specialized Layer agents should be stopped individually");
        Ok(())
    }

    /// Get agent by name
    pub async fn get_agent_by_name(&self, name: &str) -> Option<Uuid> {
        let registry = self.agent_registry.read().await;
        registry.get(name).copied()
    }

    /// Get all agent IDs
    pub async fn get_all_agent_ids(&self) -> Vec<Uuid> {
        let agents = self.agents.read().await;
        agents.keys().copied().collect()
    }

    /// Get agent count
    pub async fn agent_count(&self) -> usize {
        let agents = self.agents.read().await;
        agents.len()
    }

    /// Get comprehensive status of all agents
    pub async fn get_layer_status(&self) -> AgentResult<SpecializedLayerStatus> {
        let agents = self.agents.read().await;
        let mut agent_statuses = HashMap::new();
        let mut active_count = 0;
        let total_tasks = 0; // Task counting would require additional tracking

        for (id, agent) in agents.iter() {
            let status = agent.state().await;

            // Convert AgentStatus to JSON value for storage
            let status_json = serde_json::to_value(&status).unwrap_or_default();

            // Count active agents (Active or Busy status)
            if matches!(status, AgentStatus::Active | AgentStatus::Busy) {
                active_count += 1;
            }

            agent_statuses.insert(*id, status_json);
        }

        Ok(SpecializedLayerStatus {
            total_agents: agents.len(),
            active_agents: active_count,
            total_tasks,
            agent_statuses,
            layer_health: if active_count == agents.len() {
                100.0
            } else {
                (active_count as f64 / agents.len() as f64) * 100.0
            },
        })
    }

    /// Find the appropriate agent for a given task
    /// Note: This is a placeholder implementation. In practice, task routing
    /// would be based on task metadata, agent capabilities, and load balancing.
    async fn find_agent_for_task(&self, task: &Task) -> AgentResult<AgentId> {
        let agents = self.agents.read().await;

        // Simple agent selection based on task type or tags
        // This is a basic implementation - could be enhanced with more sophisticated routing
        for (id, agent) in agents.iter() {
            let capabilities = agent.capabilities();

            // Check if any of the agent's capabilities match the task requirements
            // In a real implementation, this would check task.tags or task.required_capabilities
            if !capabilities.is_empty() {
                return Ok(*id);
            }
        }

        // If no specific match, return the first available agent
        if let Some((id, _)) = agents.iter().next() {
            return Ok(*id);
        }

        Err(anyhow::anyhow!("No agent found for task: {}", task.name))
    }

    /// Broadcast message to all specialized agents
    pub async fn broadcast_message(&self, message: AgentMessage) -> AgentResult<()> {
        let mut agents = self.agents.write().await;
        for (id, agent) in agents.iter_mut() {
            if let Err(e) = agent.handle_message(message.clone()).await {
                error!(
                    "Failed to handle message for agent: {} ({}): {}",
                    agent.metadata().name,
                    id,
                    e
                );
            }
        }
        Ok(())
    }

    /// Get specific agent capabilities
    pub async fn get_agent_capabilities(&self, agent_name: &str) -> AgentResult<Vec<String>> {
        let agent_id = self
            .get_agent_by_name(agent_name)
            .await
            .ok_or_else(|| anyhow::anyhow!("Agent not found: {}", agent_name))?;

        let agents = self.agents.read().await;
        let agent = agents
            .get(&agent_id)
            .ok_or_else(|| anyhow::anyhow!("Agent not found: {}", agent_name))?;

        Ok(agent.capabilities().to_vec())
    }

    /// List all available agent names
    pub async fn list_agent_names(&self) -> Vec<String> {
        let registry = self.agent_registry.read().await;
        registry.keys().cloned().collect()
    }

    /// Execute a task on a specific agent by name
    pub async fn execute_task_on_agent(
        &self,
        agent_name: &str,
        task: Task,
    ) -> AgentResult<TaskResult> {
        let agent_id = self
            .get_agent_by_name(agent_name)
            .await
            .ok_or_else(|| anyhow::anyhow!("Agent not found: {}", agent_name))?;

        let mut agents = self.agents.write().await;
        let agent = agents
            .get_mut(&agent_id)
            .ok_or_else(|| anyhow::anyhow!("Agent not found: {}", agent_name))?;

        agent.execute_task(task).await
    }
}

/// Status information for the entire Specialized Layer
#[derive(Debug, serde::Serialize)]
pub struct SpecializedLayerStatus {
    pub total_agents: usize,
    pub active_agents: usize,
    pub total_tasks: u64,
    pub agent_statuses: HashMap<Uuid, serde_json::Value>,
    pub layer_health: f64, // Percentage of agents that are active
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_specialized_layer_creation() {
        let layer = SpecializedLayer::new()
            .await
            .expect("Failed to create specialized layer");
        // 8 original agents + 11 Rust/Cargo sub-agents = 19 total
        assert_eq!(layer.agent_count().await, 19);
    }

    #[tokio::test]
    async fn test_agent_lookup() {
        let layer = SpecializedLayer::new()
            .await
            .expect("Failed to create specialized layer");

        let code_gen_id = layer.get_agent_by_name("code_generation").await;
        assert!(code_gen_id.is_some());

        let security_id = layer.get_agent_by_name("security_specialist").await;
        assert!(security_id.is_some());

        let analytics_id = layer.get_agent_by_name("data_analytics").await;
        assert!(analytics_id.is_some());

        let integration_id = layer.get_agent_by_name("integration").await;
        assert!(integration_id.is_some());

        // Test Rust sub-agents
        let rust_scanner_id = layer.get_agent_by_name("rust_crate_scanner").await;
        assert!(rust_scanner_id.is_some());

        let cargo_build_id = layer.get_agent_by_name("cargo_build").await;
        assert!(cargo_build_id.is_some());

        let nonexistent_id = layer.get_agent_by_name("nonexistent").await;
        assert!(nonexistent_id.is_none());
    }

    #[tokio::test]
    async fn test_agent_names_list() {
        let layer = SpecializedLayer::new()
            .await
            .expect("Failed to create specialized layer");
        let agent_names = layer.list_agent_names().await;

        // 8 original + 11 Rust sub-agents = 19 total
        assert_eq!(agent_names.len(), 19);

        // Original agents
        assert!(agent_names.contains(&"code_generation".to_string()));
        assert!(agent_names.contains(&"testing".to_string()));
        assert!(agent_names.contains(&"deployment".to_string()));
        assert!(agent_names.contains(&"monitoring".to_string()));
        assert!(agent_names.contains(&"learning".to_string()));
        assert!(agent_names.contains(&"security_specialist".to_string()));
        assert!(agent_names.contains(&"data_analytics".to_string()));
        assert!(agent_names.contains(&"integration".to_string()));

        // Rust/Cargo sub-agents
        assert!(agent_names.contains(&"rust_crate_scanner".to_string()));
        assert!(agent_names.contains(&"cargo_build".to_string()));
        assert!(agent_names.contains(&"cargo_audit".to_string()));
        assert!(agent_names.contains(&"cargo_license".to_string()));
        assert!(agent_names.contains(&"rust_clippy".to_string()));
        assert!(agent_names.contains(&"rust_fmt".to_string()));
        assert!(agent_names.contains(&"rust_doc".to_string()));
        assert!(agent_names.contains(&"rust_ffi".to_string()));
        assert!(agent_names.contains(&"rust_wasm".to_string()));
        assert!(agent_names.contains(&"rust_cross".to_string()));
        assert!(agent_names.contains(&"rust_release".to_string()));
    }

    #[tokio::test]
    async fn test_layer_status() {
        let layer = SpecializedLayer::new()
            .await
            .expect("Failed to create specialized layer");
        let status = layer
            .get_layer_status()
            .await
            .expect("Failed to get layer status");

        // 8 original + 11 Rust sub-agents = 19 total
        assert_eq!(status.total_agents, 19);
        assert_eq!(status.agent_statuses.len(), 19);
    }
}

/// Utility functions for specialized agent management
pub mod utils {
    use super::*;

    /// Get recommended agent for specific capability name
    pub fn get_agent_for_capability(capability_name: &str) -> Option<&'static str> {
        match capability_name {
            "code_generation" => Some("code_generation"),
            "testing" => Some("testing"),
            "deployment" => Some("deployment"),
            "monitoring" => Some("monitoring"),
            "machine_learning" | "learning" => Some("learning"),
            "security_scanning" | "security" => Some("security_specialist"),
            "data_processing" | "data_analytics" => Some("data_analytics"),
            "system_integration" | "integration" => Some("integration"),
            _ => None,
        }
    }

    /// Get all agents that support a specific capability
    pub async fn get_agents_with_capability(
        layer: &SpecializedLayer,
        capability: &str,
    ) -> AgentResult<Vec<String>> {
        let mut matching_agents = Vec::new();
        let agent_names = layer.list_agent_names().await;

        for name in agent_names {
            let capabilities = layer.get_agent_capabilities(&name).await?;
            if capabilities.contains(&capability.to_string()) {
                matching_agents.push(name);
            }
        }

        Ok(matching_agents)
    }
}

// This completes the Specialized Layer implementation with all 19 domain expert agents:
//
// Original Domain Expert Agents (8):
// 1. Code Generation Agent - Multi-language automated code generation and optimization
// 2. Testing Agent - Comprehensive test automation and quality assurance
// 3. Deployment Agent - Full CI/CD pipeline and deployment orchestration
// 4. Monitoring Agent - Complete observability and system monitoring
// 5. Learning Agent - ML/AI capabilities with model training and knowledge extraction
// 6. Security Specialist Agent - Security implementation and compliance monitoring
// 7. Data Analytics Agent - Data processing, analytics, and business intelligence
// 8. Integration Agent - System integration, API management, and workflow orchestration
//
// Rust/Cargo Sub-Agents (11):
// 9. RustCrateScannerAgent - Discover crates, versions, features; map dependency tree
// 10. CargoBuildAgent - Build/bench/test workflows with caching and EFG-aware parallelism
// 11. CargoAuditAgent - Integrate cargo-audit, triage RUSTSEC advisories
// 12. CargoLicenseAgent - Scan licenses, enforce allow-lists
// 13. RustClippyAgent - Clippy linting tiers; autofix common lints
// 14. RustFmtAgent - Format code; enforce style policies
// 15. RustDocAgent - Generate and publish doc artifacts
// 16. RustFFIAgent - bindgen/cbindgen pipelines, ABI tests
// 17. RustWasmAgent - wasm-pack + size/perf budgeting, bindings
// 18. RustCrossAgent - Cross-compile matrix: musl/aarch64 etc.
// 19. RustReleaseAgent - Crate publishing workflow (private/public)
//
// All Rust/Cargo sub-agents output: artifacts, SBOM, scores, advisories
// All Rust/Cargo sub-agents enforce policies: MSRV, semver, export-control
//
// Together, these agents provide comprehensive operational capabilities to execute
// strategic decisions from the Board Layer with technical excellence and autonomous operation.
