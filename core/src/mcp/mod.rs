//! MCP (Model Context Protocol) Integration
//!
//! Provides tool execution capabilities for agents via MCP protocol.
//! Tools are routed through the agentgateway for:
//! - Centralized access control
//! - Rate limiting
//! - Observability
//! - Multi-provider routing

pub mod agent_tools;

use crate::ai::{GatewayConfig, McpTool, SharedGateway, ToolCallRequest};

pub use agent_tools::{ToolEnabledAgent, ToolEnabledAgentFactory};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub success: bool,
    pub output: serde_json::Value,
    pub execution_time_ms: u64,
}

/// Tool registry for managing available tools
pub struct ToolRegistry {
    gateway: SharedGateway,
    tools: RwLock<HashMap<String, McpTool>>,
    tool_handlers: RwLock<HashMap<String, Arc<dyn ToolHandler>>>,
}

/// Trait for custom tool handlers
#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    /// Execute the tool with given arguments
    async fn execute(&self, args: serde_json::Value) -> Result<serde_json::Value>;

    /// Get tool metadata
    fn metadata(&self) -> McpTool;
}

impl ToolRegistry {
    /// Create a new tool registry connected to the gateway
    pub fn new(gateway: SharedGateway) -> Self {
        Self {
            gateway,
            tools: RwLock::new(HashMap::new()),
            tool_handlers: RwLock::new(HashMap::new()),
        }
    }

    /// Create with default gateway configuration
    pub fn with_default_gateway() -> Result<Self> {
        let gateway = crate::ai::create_gateway(GatewayConfig::default())?;
        Ok(Self::new(gateway))
    }

    /// Refresh tools from gateway
    pub async fn refresh(&self) -> Result<()> {
        let tools = self.gateway.list_tools().await?;
        let mut cache = self.tools.write().await;
        cache.clear();
        for tool in tools {
            cache.insert(tool.name.clone(), tool);
        }
        Ok(())
    }

    /// Register a local tool handler
    pub async fn register_handler(&self, handler: Arc<dyn ToolHandler>) {
        let metadata = handler.metadata();
        let name = metadata.name.clone();

        self.tools.write().await.insert(name.clone(), metadata);
        self.tool_handlers.write().await.insert(name, handler);
    }

    /// List all available tools
    pub async fn list_tools(&self) -> Vec<McpTool> {
        self.tools.read().await.values().cloned().collect()
    }

    /// Get a specific tool
    pub async fn get_tool(&self, name: &str) -> Option<McpTool> {
        self.tools.read().await.get(name).cloned()
    }

    /// Execute a tool
    pub async fn execute(&self, tool_name: &str, args: serde_json::Value) -> Result<ToolResult> {
        let start = std::time::Instant::now();

        // Check for local handler first
        let handlers = self.tool_handlers.read().await;
        if let Some(handler) = handlers.get(tool_name) {
            let output = handler.execute(args).await?;
            return Ok(ToolResult {
                tool_name: tool_name.to_string(),
                success: true,
                output,
                execution_time_ms: start.elapsed().as_millis() as u64,
            });
        }
        drop(handlers);

        // Fall back to gateway
        let request = ToolCallRequest {
            tool: tool_name.to_string(),
            arguments: args,
        };

        let response = self.gateway.call_tool(request).await?;

        Ok(ToolResult {
            tool_name: tool_name.to_string(),
            success: !response.is_error,
            output: response.content,
            execution_time_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute multiple tools in parallel
    pub async fn execute_batch(
        &self,
        calls: Vec<(String, serde_json::Value)>,
    ) -> Vec<Result<ToolResult>> {
        let futures: Vec<_> = calls
            .into_iter()
            .map(|(tool, args)| {
                let registry = self;
                async move { registry.execute(&tool, args).await }
            })
            .collect();

        futures::future::join_all(futures).await
    }
}

/// Agent tool context - provides tools to an agent
pub struct AgentToolContext {
    agent_id: String,
    registry: Arc<ToolRegistry>,
    allowed_tools: Option<Vec<String>>,
}

impl AgentToolContext {
    /// Create a new tool context for an agent
    pub fn new(agent_id: impl Into<String>, registry: Arc<ToolRegistry>) -> Self {
        Self {
            agent_id: agent_id.into(),
            registry,
            allowed_tools: None,
        }
    }

    /// Restrict to specific tools
    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = Some(tools);
        self
    }

    /// Check if a tool is allowed for this agent
    pub fn is_allowed(&self, tool_name: &str) -> bool {
        match &self.allowed_tools {
            Some(allowed) => allowed.contains(&tool_name.to_string()),
            None => true, // All tools allowed if no restriction
        }
    }

    /// List available tools for this agent
    pub async fn available_tools(&self) -> Vec<McpTool> {
        let all_tools = self.registry.list_tools().await;

        match &self.allowed_tools {
            Some(allowed) => all_tools
                .into_iter()
                .filter(|t| allowed.contains(&t.name))
                .collect(),
            None => all_tools,
        }
    }

    /// Execute a tool
    pub async fn execute(&self, tool_name: &str, args: serde_json::Value) -> Result<ToolResult> {
        if !self.is_allowed(tool_name) {
            return Err(anyhow!(
                "Tool '{}' not allowed for agent '{}'",
                tool_name,
                self.agent_id
            ));
        }

        tracing::info!(
            agent_id = %self.agent_id,
            tool = %tool_name,
            "Agent executing tool"
        );

        self.registry.execute(tool_name, args).await
    }
}

/// Built-in tool: Echo (for testing)
pub struct EchoTool;

#[async_trait::async_trait]
impl ToolHandler for EchoTool {
    async fn execute(&self, args: serde_json::Value) -> Result<serde_json::Value> {
        Ok(args)
    }

    fn metadata(&self) -> McpTool {
        McpTool {
            name: "echo".to_string(),
            description: "Echoes back the input arguments".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "message": { "type": "string" }
                }
            }),
        }
    }
}

/// Built-in tool: Shell command execution
pub struct ShellTool {
    allowed_commands: Option<Vec<String>>,
}

impl ShellTool {
    pub fn new() -> Self {
        Self {
            allowed_commands: None,
        }
    }

    pub fn with_allowed_commands(mut self, commands: Vec<String>) -> Self {
        self.allowed_commands = Some(commands);
        self
    }
}

impl Default for ShellTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ToolHandler for ShellTool {
    async fn execute(&self, args: serde_json::Value) -> Result<serde_json::Value> {
        let command = args
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'command' argument"))?;

        // Security check
        if let Some(allowed) = &self.allowed_commands {
            let cmd_name = command.split_whitespace().next().unwrap_or("");
            if !allowed.iter().any(|a| a == cmd_name) {
                return Err(anyhow!("Command '{}' not allowed", cmd_name));
            }
        }

        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .await?;

        Ok(serde_json::json!({
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "exit_code": output.status.code()
        }))
    }

    fn metadata(&self) -> McpTool {
        McpTool {
            name: "shell".to_string(),
            description: "Execute shell commands".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["command"],
                "properties": {
                    "command": { "type": "string", "description": "Shell command to execute" }
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo_tool() {
        let tool = EchoTool;
        let input = serde_json::json!({"message": "hello"});
        let result = tool.execute(input.clone()).await.unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_tool_metadata() {
        let tool = EchoTool;
        let meta = tool.metadata();
        assert_eq!(meta.name, "echo");
    }

    #[test]
    fn test_agent_tool_context_allowed() {
        let gateway = crate::ai::create_gateway(GatewayConfig::default()).unwrap();
        let registry = Arc::new(ToolRegistry::new(gateway));
        let ctx = AgentToolContext::new("test-agent", registry)
            .with_allowed_tools(vec!["echo".to_string()]);

        assert!(ctx.is_allowed("echo"));
        assert!(!ctx.is_allowed("shell"));
    }
}
