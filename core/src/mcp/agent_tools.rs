//! Agent Tool Integration
//!
//! Extends agents with MCP tool capabilities without modifying the core Agent trait.

use crate::agents::{Agent, AgentMessage, AgentResult, ManagedAgent};
use crate::mcp::{AgentToolContext, ToolRegistry, ToolResult};
use crate::orchestration::Task;
use agentaskit_shared::{AgentMetadata, AgentStatus, HealthStatus, TaskResult, TaskStatus};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Agent with tool execution capabilities
pub struct ToolEnabledAgent {
    inner: ManagedAgent,
    tool_context: AgentToolContext,
}

impl ToolEnabledAgent {
    /// Create a tool-enabled agent
    pub fn new(agent: ManagedAgent, registry: Arc<ToolRegistry>) -> Self {
        let agent_id = agent.id.to_string();
        let tool_context = AgentToolContext::new(agent_id, registry);

        Self {
            inner: agent,
            tool_context,
        }
    }

    /// Create with restricted tools
    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.tool_context = AgentToolContext::new(
            self.inner.id.to_string(),
            Arc::new(ToolRegistry::with_default_gateway().unwrap()),
        )
        .with_allowed_tools(tools);
        self
    }

    /// Execute a tool
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        args: serde_json::Value,
    ) -> Result<ToolResult> {
        self.tool_context.execute(tool_name, args).await
    }

    /// List available tools for this agent
    pub async fn available_tools(&self) -> Vec<crate::mcp::McpTool> {
        self.tool_context.available_tools().await
    }

    /// Get inner agent
    pub fn inner(&self) -> &ManagedAgent {
        &self.inner
    }

    /// Get mutable inner agent
    pub fn inner_mut(&mut self) -> &mut ManagedAgent {
        &mut self.inner
    }
}

#[async_trait]
impl Agent for ToolEnabledAgent {
    async fn start(&mut self) -> AgentResult<()> {
        self.inner.start().await
    }

    async fn stop(&mut self) -> AgentResult<()> {
        self.inner.stop().await
    }

    async fn handle_message(&mut self, message: AgentMessage) -> AgentResult<Option<AgentMessage>> {
        self.inner.handle_message(message).await
    }

    async fn execute_task(&mut self, task: Task) -> AgentResult<TaskResult> {
        // Check if task requires tool execution
        if let Some(tool_calls) = task.input_data.get("tool_calls") {
            if let Some(calls) = tool_calls.as_array() {
                let mut results = Vec::new();

                for call in calls {
                    let tool_name = call
                        .get("tool")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let args = call
                        .get("arguments")
                        .cloned()
                        .unwrap_or(serde_json::json!({}));

                    match self.execute_tool(tool_name, args).await {
                        Ok(result) => results.push(serde_json::json!({
                            "tool": tool_name,
                            "success": result.success,
                            "output": result.output,
                            "time_ms": result.execution_time_ms
                        })),
                        Err(e) => results.push(serde_json::json!({
                            "tool": tool_name,
                            "success": false,
                            "error": e.to_string()
                        })),
                    }
                }

                return Ok(TaskResult {
                    task_id: task.id,
                    status: TaskStatus::Completed,
                    output_data: Some(serde_json::json!({ "tool_results": results })),
                    error_message: None,
                    completed_at: chrono::Utc::now(),
                });
            }
        }

        // Delegate to inner agent for non-tool tasks
        self.inner.execute_task(task).await
    }

    async fn health_check(&self) -> AgentResult<HealthStatus> {
        self.inner.health_check().await
    }

    async fn update_config(&mut self, config: serde_json::Value) -> AgentResult<()> {
        self.inner.update_config(config).await
    }

    fn capabilities(&self) -> &[String] {
        self.inner.capabilities()
    }

    async fn state(&self) -> AgentStatus {
        self.inner.state().await
    }

    fn metadata(&self) -> &AgentMetadata {
        self.inner.metadata()
    }
}

/// Factory for creating tool-enabled agents
pub struct ToolEnabledAgentFactory {
    registry: Arc<ToolRegistry>,
}

impl ToolEnabledAgentFactory {
    pub fn new(registry: Arc<ToolRegistry>) -> Self {
        Self { registry }
    }

    pub fn create(&self, agent: ManagedAgent) -> ToolEnabledAgent {
        ToolEnabledAgent::new(agent, Arc::clone(&self.registry))
    }

    pub fn create_with_tools(
        &self,
        agent: ManagedAgent,
        allowed_tools: Vec<String>,
    ) -> ToolEnabledAgent {
        ToolEnabledAgent::new(agent, Arc::clone(&self.registry))
            .with_allowed_tools(allowed_tools)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::AgentLayer;
    use agentaskit_shared::ResourceRequirements;

    #[tokio::test]
    async fn test_tool_enabled_agent_creation() {
        let agent = ManagedAgent::new(
            "test-agent".to_string(),
            AgentLayer::Micro,
            vec!["task_execution".to_string()],
            ResourceRequirements::default(),
        );

        let registry = Arc::new(ToolRegistry::with_default_gateway().unwrap());
        let tool_agent = ToolEnabledAgent::new(agent, registry);

        assert_eq!(tool_agent.inner().name, "test-agent");
    }
}
