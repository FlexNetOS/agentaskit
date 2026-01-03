//! DeploymentManagementProtocol Implementation

use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use shared::protocols::{DeploymentManagementProtocol, DeploymentStatus};
use shared::data_models::{AgentId, DeploymentManifestEntry};
use crate::agents::AgentManager;

/// Concrete implementation of DeploymentManagementProtocol
pub struct DeploymentManagementService {
    agent_manager: Arc<AgentManager>,
    deployments: Arc<RwLock<HashMap<AgentId, DeploymentStatus>>>,
}

impl DeploymentManagementService {
    pub fn new(agent_manager: Arc<AgentManager>) -> Self {
        Self {
            agent_manager,
            deployments: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DeploymentManagementProtocol for DeploymentManagementService {
    async fn deploy_agent(&self, manifest_entry: DeploymentManifestEntry) -> Result<AgentId> {
        // Create agent from manifest
        let agent = self.agent_manager.create_agent(
            manifest_entry.agent_metadata.name.clone(),
            crate::agents::AgentLayer::Specialized,
            manifest_entry.agent_metadata.capabilities.clone(),
        ).await?;

        let agent_id = agent.id;

        // Track deployment
        let status = DeploymentStatus {
            agent_id,
            deployment_id: Uuid::new_v4(),
            status: "deployed".to_string(),
            instances: 1,
            target_instances: manifest_entry.instances,
            healthy_instances: 1,
            last_updated: chrono::Utc::now(),
            deployment_config: manifest_entry.config.clone(),
        };

        let mut deployments = self.deployments.write().await;
        deployments.insert(agent_id, status);

        // Start the agent
        self.agent_manager.start_agent(agent_id).await?;

        Ok(agent_id)
    }

    async fn undeploy_agent(&self, agent_id: AgentId) -> Result<()> {
        // Stop and remove agent
        self.agent_manager.stop_agent(agent_id).await?;

        // Remove from deployments
        let mut deployments = self.deployments.write().await;
        deployments.remove(&agent_id);

        Ok(())
    }

    async fn scale_agent(&self, agent_id: AgentId, target_instances: u32) -> Result<()> {
        let mut deployments = self.deployments.write().await;
        if let Some(status) = deployments.get_mut(&agent_id) {
            status.target_instances = target_instances;
            status.last_updated = chrono::Utc::now();
            status.status = "scaling".to_string();
        }
        Ok(())
    }

    async fn update_deployment(&self, agent_id: AgentId, manifest_entry: DeploymentManifestEntry) -> Result<()> {
        let mut deployments = self.deployments.write().await;
        if let Some(status) = deployments.get_mut(&agent_id) {
            status.target_instances = manifest_entry.instances;
            status.deployment_config = manifest_entry.config;
            status.last_updated = chrono::Utc::now();
            status.status = "updating".to_string();
        }
        Ok(())
    }

    async fn get_deployment_status(&self, agent_id: AgentId) -> Result<DeploymentStatus> {
        let deployments = self.deployments.read().await;
        deployments.get(&agent_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Deployment not found for agent: {}", agent_id))
    }

    async fn list_deployments(&self) -> Result<Vec<DeploymentStatus>> {
        let deployments = self.deployments.read().await;
        Ok(deployments.values().cloned().collect())
    }
}
