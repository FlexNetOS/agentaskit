//! AgentLifecycleProtocol Implementation

use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;

use shared::protocols::{AgentLifecycleProtocol, AgentFilter};
use shared::data_models::{AgentId, AgentMetadata};
use crate::agents::AgentManager;

/// Concrete implementation of AgentLifecycleProtocol
pub struct AgentLifecycleService {
    agent_manager: Arc<AgentManager>,
}

impl AgentLifecycleService {
    pub fn new(agent_manager: Arc<AgentManager>) -> Self {
        Self { agent_manager }
    }
}

#[async_trait]
impl AgentLifecycleProtocol for AgentLifecycleService {
    async fn initialize_agent(&self, metadata: AgentMetadata) -> Result<AgentId> {
        let agent = self.agent_manager.create_agent(
            metadata.name.clone(),
            crate::agents::AgentLayer::Specialized,
            metadata.capabilities.clone(),
        ).await?;
        Ok(agent.id)
    }

    async fn start_agent(&self, agent_id: AgentId) -> Result<()> {
        self.agent_manager.start_agent(agent_id).await
    }

    async fn stop_agent(&self, agent_id: AgentId) -> Result<()> {
        self.agent_manager.stop_agent(agent_id).await
    }

    async fn force_shutdown_agent(&self, agent_id: AgentId) -> Result<()> {
        self.agent_manager.force_shutdown_agent(agent_id).await
    }

    async fn update_agent_metadata(&self, agent_id: AgentId, metadata: AgentMetadata) -> Result<()> {
        self.agent_manager.update_agent(agent_id, |agent| {
            agent.name = metadata.name.clone();
            agent.capabilities = metadata.capabilities.clone();
        }).await
    }

    async fn get_agent_metadata(&self, agent_id: AgentId) -> Result<AgentMetadata> {
        let agent = self.agent_manager.get_agent(agent_id).await?;
        Ok(AgentMetadata {
            id: Some(agent.id),
            name: agent.name,
            agent_type: agent.agent_type,
            description: String::new(),
            version: "1.0.0".to_string(),
            capabilities: agent.capabilities,
            resource_requirements: serde_json::json!({}),
            config: serde_json::json!({}),
            tags: std::collections::HashMap::new(),
        })
    }

    async fn list_agents(&self, filter: Option<AgentFilter>) -> Result<Vec<AgentMetadata>> {
        let agents = self.agent_manager.list_agents().await?;

        let filtered: Vec<_> = agents.into_iter()
            .filter(|agent| {
                if let Some(ref f) = filter {
                    if let Some(ref agent_type) = f.agent_type {
                        if &agent.agent_type != agent_type {
                            return false;
                        }
                    }
                    if let Some(ref caps) = f.capabilities {
                        if !caps.iter().all(|c| agent.capabilities.contains(c)) {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|agent| AgentMetadata {
                id: Some(agent.id),
                name: agent.name,
                agent_type: agent.agent_type,
                description: String::new(),
                version: "1.0.0".to_string(),
                capabilities: agent.capabilities,
                resource_requirements: serde_json::json!({}),
                config: serde_json::json!({}),
                tags: std::collections::HashMap::new(),
            })
            .collect();

        Ok(filtered)
    }
}
