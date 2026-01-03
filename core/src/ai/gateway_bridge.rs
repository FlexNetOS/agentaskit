//! Agent Gateway Bridge
//!
//! Connects the core system to the agentgateway for:
//! - MCP protocol routing
//! - A2A agent communication
//! - Provider load balancing
//! - Tool execution

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Gateway connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Gateway MCP endpoint
    pub mcp_endpoint: String,
    /// Gateway A2A endpoint
    pub a2a_endpoint: String,
    /// Connection timeout in seconds
    pub timeout_secs: u64,
    /// Enable TLS
    pub tls_enabled: bool,
    /// Auth token (optional)
    pub auth_token: Option<String>,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            mcp_endpoint: "http://127.0.0.1:8080".to_string(),
            a2a_endpoint: "http://127.0.0.1:8081".to_string(),
            timeout_secs: 30,
            tls_enabled: false,
            auth_token: None,
        }
    }
}

/// MCP Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// MCP Tool call request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    pub tool: String,
    pub arguments: serde_json::Value,
}

/// MCP Tool call response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResponse {
    pub content: serde_json::Value,
    pub is_error: bool,
}

/// A2A Agent card (from Agent2Agent protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    pub id: String,
    pub name: String,
    pub description: String,
    pub skills: Vec<AgentSkill>,
    pub endpoint: String,
}

/// Agent skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub input_schema: Option<serde_json::Value>,
}

/// Gateway bridge for connecting to agentgateway
pub struct GatewayBridge {
    config: GatewayConfig,
    client: reqwest::Client,
    tools_cache: RwLock<HashMap<String, McpTool>>,
    agents_cache: RwLock<HashMap<String, AgentCard>>,
}

impl GatewayBridge {
    /// Create a new gateway bridge
    pub fn new(config: GatewayConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()?;

        Ok(Self {
            config,
            client,
            tools_cache: RwLock::new(HashMap::new()),
            agents_cache: RwLock::new(HashMap::new()),
        })
    }

    /// Create with default configuration
    pub fn default_config() -> Result<Self> {
        Self::new(GatewayConfig::default())
    }

    /// Check if gateway is available
    pub async fn is_available(&self) -> bool {
        let health_url = format!("{}/health", self.config.mcp_endpoint);
        self.client.get(&health_url).send().await.is_ok()
    }

    /// List available MCP tools
    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let url = format!("{}/tools/list", self.config.mcp_endpoint);

        let mut request = self.client.post(&url);
        if let Some(token) = &self.config.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .json(&serde_json::json!({}))
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            let tools: Vec<McpTool> = serde_json::from_value(
                result.get("tools").cloned().unwrap_or(serde_json::json!([]))
            )?;

            // Update cache
            let mut cache = self.tools_cache.write().await;
            for tool in &tools {
                cache.insert(tool.name.clone(), tool.clone());
            }

            Ok(tools)
        } else {
            Err(anyhow!("Failed to list tools: {}", response.status()))
        }
    }

    /// Call an MCP tool
    pub async fn call_tool(&self, request: ToolCallRequest) -> Result<ToolCallResponse> {
        let url = format!("{}/tools/call", self.config.mcp_endpoint);

        let mut http_request = self.client.post(&url);
        if let Some(token) = &self.config.auth_token {
            http_request = http_request.header("Authorization", format!("Bearer {}", token));
        }

        let body = serde_json::json!({
            "name": request.tool,
            "arguments": request.arguments
        });

        let response = http_request
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            Ok(ToolCallResponse {
                content: result.get("content").cloned().unwrap_or(serde_json::json!(null)),
                is_error: result.get("isError").and_then(|v| v.as_bool()).unwrap_or(false),
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Ok(ToolCallResponse {
                content: serde_json::json!({ "error": error_text }),
                is_error: true,
            })
        }
    }

    /// List available A2A agents
    pub async fn list_agents(&self) -> Result<Vec<AgentCard>> {
        let url = format!("{}/.well-known/agent.json", self.config.a2a_endpoint);

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let agents: Vec<AgentCard> = response.json().await?;

            // Update cache
            let mut cache = self.agents_cache.write().await;
            for agent in &agents {
                cache.insert(agent.id.clone(), agent.clone());
            }

            Ok(agents)
        } else {
            Ok(vec![])
        }
    }

    /// Send a task to an A2A agent
    pub async fn send_task(
        &self,
        agent_id: &str,
        skill_id: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let cache = self.agents_cache.read().await;
        let agent = cache
            .get(agent_id)
            .ok_or_else(|| anyhow!("Agent not found: {}", agent_id))?;

        let url = format!("{}/tasks/send", agent.endpoint);

        let body = serde_json::json!({
            "skill_id": skill_id,
            "input": input
        });

        let mut request = self.client.post(&url);
        if let Some(token) = &self.config.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.json(&body).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow!("Task failed: {}", response.status()))
        }
    }

    /// Get a tool from cache
    pub async fn get_tool(&self, name: &str) -> Option<McpTool> {
        self.tools_cache.read().await.get(name).cloned()
    }

    /// Get an agent from cache
    pub async fn get_agent(&self, id: &str) -> Option<AgentCard> {
        self.agents_cache.read().await.get(id).cloned()
    }
}

/// Shared gateway instance
pub type SharedGateway = Arc<GatewayBridge>;

/// Create a shared gateway bridge
pub fn create_gateway(config: GatewayConfig) -> Result<SharedGateway> {
    Ok(Arc::new(GatewayBridge::new(config)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GatewayConfig::default();
        assert_eq!(config.mcp_endpoint, "http://127.0.0.1:8080");
        assert_eq!(config.a2a_endpoint, "http://127.0.0.1:8081");
        assert!(!config.tls_enabled);
    }

    #[test]
    fn test_tool_call_request() {
        let request = ToolCallRequest {
            tool: "calculator".to_string(),
            arguments: serde_json::json!({ "a": 1, "b": 2 }),
        };
        assert_eq!(request.tool, "calculator");
    }

    #[tokio::test]
    async fn test_gateway_bridge_creation() {
        let config = GatewayConfig::default();
        let bridge = GatewayBridge::new(config);
        assert!(bridge.is_ok());
    }
}
