//! Model Context Protocol (MCP) integration
//!
//! This module provides MCP client functionality for connecting to
//! MCP servers through agentgateway.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP client for communicating with MCP servers via gateway
pub struct McpClient {
    base_url: String,
    client: reqwest::Client,
}

impl McpClient {
    /// Create a new MCP client
    pub fn new(gateway_address: &str, target_name: &str) -> Self {
        Self {
            base_url: format!("http://{}/mcp/{}", gateway_address, target_name),
            client: reqwest::Client::new(),
        }
    }

    /// Create a new MCP client with SSE transport
    pub fn new_sse(gateway_address: &str, target_name: &str) -> Self {
        Self {
            base_url: format!("http://{}/sse/{}", gateway_address, target_name),
            client: reqwest::Client::new(),
        }
    }

    /// List available tools from the MCP server
    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let response: McpResponse<ToolsResult> = self.client
            .post(&format!("{}/tools/list", self.base_url))
            .json(&McpRequest {
                jsonrpc: "2.0".to_string(),
                method: "tools/list".to_string(),
                params: serde_json::json!({}),
                id: 1,
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(response.result.tools)
    }

    /// Call a tool on the MCP server
    pub async fn call_tool(&self, name: &str, arguments: serde_json::Value) -> Result<McpToolResult> {
        let response: McpResponse<McpToolResult> = self.client
            .post(&format!("{}/tools/call", self.base_url))
            .json(&McpRequest {
                jsonrpc: "2.0".to_string(),
                method: "tools/call".to_string(),
                params: serde_json::json!({
                    "name": name,
                    "arguments": arguments,
                }),
                id: 1,
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(response.result)
    }

    /// List available resources
    pub async fn list_resources(&self) -> Result<Vec<McpResource>> {
        let response: McpResponse<ResourcesResult> = self.client
            .post(&format!("{}/resources/list", self.base_url))
            .json(&McpRequest {
                jsonrpc: "2.0".to_string(),
                method: "resources/list".to_string(),
                params: serde_json::json!({}),
                id: 1,
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(response.result.resources)
    }

    /// Read a resource
    pub async fn read_resource(&self, uri: &str) -> Result<McpResourceContent> {
        let response: McpResponse<McpResourceContent> = self.client
            .post(&format!("{}/resources/read", self.base_url))
            .json(&McpRequest {
                jsonrpc: "2.0".to_string(),
                method: "resources/read".to_string(),
                params: serde_json::json!({
                    "uri": uri,
                }),
                id: 1,
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(response.result)
    }

    /// List available prompts
    pub async fn list_prompts(&self) -> Result<Vec<McpPrompt>> {
        let response: McpResponse<PromptsResult> = self.client
            .post(&format!("{}/prompts/list", self.base_url))
            .json(&McpRequest {
                jsonrpc: "2.0".to_string(),
                method: "prompts/list".to_string(),
                params: serde_json::json!({}),
                id: 1,
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(response.result.prompts)
    }

    /// Get a prompt with arguments
    pub async fn get_prompt(&self, name: &str, arguments: HashMap<String, String>) -> Result<McpPromptMessages> {
        let response: McpResponse<McpPromptMessages> = self.client
            .post(&format!("{}/prompts/get", self.base_url))
            .json(&McpRequest {
                jsonrpc: "2.0".to_string(),
                method: "prompts/get".to_string(),
                params: serde_json::json!({
                    "name": name,
                    "arguments": arguments,
                }),
                id: 1,
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(response.result)
    }
}

/// MCP JSON-RPC request
#[derive(Debug, Serialize)]
struct McpRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: u64,
}

/// MCP JSON-RPC response
#[derive(Debug, Deserialize)]
struct McpResponse<T> {
    jsonrpc: String,
    result: T,
    id: u64,
}

/// MCP tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub input_schema: Option<serde_json::Value>,
}

/// Result of listing tools
#[derive(Debug, Deserialize)]
struct ToolsResult {
    tools: Vec<McpTool>,
}

/// MCP tool call result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolResult {
    #[serde(default)]
    pub content: Vec<McpContent>,
    #[serde(default)]
    pub is_error: bool,
}

/// MCP content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpContent {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub data: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
}

/// MCP resource definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
}

/// Result of listing resources
#[derive(Debug, Deserialize)]
struct ResourcesResult {
    resources: Vec<McpResource>,
}

/// MCP resource content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceContent {
    pub contents: Vec<McpContent>,
}

/// MCP prompt definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPrompt {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<McpPromptArgument>,
}

/// MCP prompt argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptArgument {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
}

/// Result of listing prompts
#[derive(Debug, Deserialize)]
struct PromptsResult {
    prompts: Vec<McpPrompt>,
}

/// MCP prompt messages result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptMessages {
    #[serde(default)]
    pub description: Option<String>,
    pub messages: Vec<McpMessage>,
}

/// MCP message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMessage {
    pub role: String,
    pub content: McpContent,
}

/// MCP server configuration for stdio targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Server name
    pub name: String,
    /// Command to run
    pub command: String,
    /// Command arguments
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variables
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl McpServerConfig {
    /// Create a new stdio MCP server configuration
    pub fn new_stdio(name: &str, command: &str) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            args: Vec::new(),
            env: HashMap::new(),
        }
    }

    /// Add arguments
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Add environment variable
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_client_new() {
        let client = McpClient::new("localhost:8080", "test-server");
        assert_eq!(client.base_url, "http://localhost:8080/mcp/test-server");
    }

    #[test]
    fn test_mcp_server_config() {
        let config = McpServerConfig::new_stdio("my-server", "npx")
            .with_args(vec!["-y".to_string(), "@modelcontextprotocol/server-everything".to_string()])
            .with_env("DEBUG", "true");

        assert_eq!(config.name, "my-server");
        assert_eq!(config.command, "npx");
        assert_eq!(config.args.len(), 2);
        assert_eq!(config.env.get("DEBUG"), Some(&"true".to_string()));
    }
}
