//! CapabilityManagementProtocol Implementation

use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use shared::protocols::CapabilityManagementProtocol;
use shared::data_models::{AgentId, CapabilityToken};
use crate::security::SecurityManager;

/// Concrete implementation of CapabilityManagementProtocol
pub struct CapabilityManagementService {
    security_manager: Arc<SecurityManager>,
    tokens: Arc<RwLock<HashMap<Uuid, CapabilityToken>>>,
    agent_capabilities: Arc<RwLock<HashMap<AgentId, Vec<Uuid>>>>,
}

impl CapabilityManagementService {
    pub fn new(security_manager: Arc<SecurityManager>) -> Self {
        Self {
            security_manager,
            tokens: Arc::new(RwLock::new(HashMap::new())),
            agent_capabilities: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl CapabilityManagementProtocol for CapabilityManagementService {
    async fn grant_capability(&self, token: CapabilityToken) -> Result<()> {
        let token_id = token.token_id;
        let agent_id = token.agent_id;

        // Store token
        let mut tokens = self.tokens.write().await;
        tokens.insert(token_id, token);

        // Track agent's capabilities
        let mut agent_caps = self.agent_capabilities.write().await;
        agent_caps.entry(agent_id).or_insert_with(Vec::new).push(token_id);

        Ok(())
    }

    async fn revoke_capability(&self, token_id: Uuid) -> Result<()> {
        let mut tokens = self.tokens.write().await;
        if let Some(token) = tokens.remove(&token_id) {
            let mut agent_caps = self.agent_capabilities.write().await;
            if let Some(caps) = agent_caps.get_mut(&token.agent_id) {
                caps.retain(|id| id != &token_id);
            }
        }
        Ok(())
    }

    async fn has_capability(&self, agent_id: AgentId, capability: String) -> Result<bool> {
        let tokens = self.tokens.read().await;
        let agent_caps = self.agent_capabilities.read().await;

        if let Some(token_ids) = agent_caps.get(&agent_id) {
            for token_id in token_ids {
                if let Some(token) = tokens.get(token_id) {
                    if token.capabilities.contains(&capability) {
                        // Check if token is still valid
                        if token.expires_at > chrono::Utc::now() {
                            return Ok(true);
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    async fn list_agent_capabilities(&self, agent_id: AgentId) -> Result<Vec<CapabilityToken>> {
        let tokens = self.tokens.read().await;
        let agent_caps = self.agent_capabilities.read().await;

        let mut result = Vec::new();
        if let Some(token_ids) = agent_caps.get(&agent_id) {
            for token_id in token_ids {
                if let Some(token) = tokens.get(token_id) {
                    result.push(token.clone());
                }
            }
        }
        Ok(result)
    }

    async fn validate_capability_token(&self, token: CapabilityToken) -> Result<bool> {
        // Check token exists and matches
        let tokens = self.tokens.read().await;
        if let Some(stored) = tokens.get(&token.token_id) {
            // Verify token hasn't expired
            if stored.expires_at <= chrono::Utc::now() {
                return Ok(false);
            }
            // Verify token matches stored version
            Ok(stored.agent_id == token.agent_id && stored.capabilities == token.capabilities)
        } else {
            Ok(false)
        }
    }
}
