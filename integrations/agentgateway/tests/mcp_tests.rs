//! MCP module tests for agentgateway integration

use std::collections::HashMap;

#[path = "../src/mcp.rs"]
mod mcp;

use mcp::*;

#[test]
fn test_mcp_tool_definition() {
    let tool = McpTool {
        name: "calculator".to_string(),
        description: Some("A simple calculator".to_string()),
        input_schema: Some(serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {"type": "string"},
                "a": {"type": "number"},
                "b": {"type": "number"}
            }
        })),
    };

    assert_eq!(tool.name, "calculator");
    assert!(tool.description.is_some());
    assert!(tool.input_schema.is_some());
}

#[test]
fn test_mcp_tool_result() {
    let result = McpToolResult {
        content: vec![
            McpContent {
                content_type: "text".to_string(),
                text: Some("Result: 42".to_string()),
                data: None,
                mime_type: None,
            }
        ],
        is_error: false,
    };

    assert!(!result.is_error);
    assert_eq!(result.content.len(), 1);
    assert_eq!(result.content[0].text.as_ref().unwrap(), "Result: 42");
}

#[test]
fn test_mcp_tool_result_error() {
    let result = McpToolResult {
        content: vec![
            McpContent {
                content_type: "text".to_string(),
                text: Some("Error: Division by zero".to_string()),
                data: None,
                mime_type: None,
            }
        ],
        is_error: true,
    };

    assert!(result.is_error);
}

#[test]
fn test_mcp_resource() {
    let resource = McpResource {
        uri: "file:///project/src/main.rs".to_string(),
        name: "main.rs".to_string(),
        description: Some("Main entry point".to_string()),
        mime_type: Some("text/x-rust".to_string()),
    };

    assert!(resource.uri.starts_with("file://"));
    assert!(resource.mime_type.is_some());
}

#[test]
fn test_mcp_prompt() {
    let prompt = McpPrompt {
        name: "code-review".to_string(),
        description: Some("Review code for issues".to_string()),
        arguments: vec![
            McpPromptArgument {
                name: "code".to_string(),
                description: Some("The code to review".to_string()),
                required: true,
            },
            McpPromptArgument {
                name: "language".to_string(),
                description: Some("Programming language".to_string()),
                required: false,
            },
        ],
    };

    assert_eq!(prompt.name, "code-review");
    assert_eq!(prompt.arguments.len(), 2);
    assert!(prompt.arguments[0].required);
    assert!(!prompt.arguments[1].required);
}

#[test]
fn test_mcp_server_config_stdio() {
    let config = McpServerConfig::new_stdio("my-server", "npx")
        .with_args(vec!["-y".to_string(), "@mcp/server-everything".to_string()])
        .with_env("DEBUG", "true")
        .with_env("LOG_LEVEL", "info");

    assert_eq!(config.name, "my-server");
    assert_eq!(config.command, "npx");
    assert_eq!(config.args.len(), 2);
    assert_eq!(config.env.len(), 2);
    assert_eq!(config.env.get("DEBUG"), Some(&"true".to_string()));
}

#[test]
fn test_mcp_content_types() {
    // Text content
    let text_content = McpContent {
        content_type: "text".to_string(),
        text: Some("Hello, world!".to_string()),
        data: None,
        mime_type: None,
    };
    assert!(text_content.text.is_some());
    assert!(text_content.data.is_none());

    // Binary content
    let binary_content = McpContent {
        content_type: "blob".to_string(),
        text: None,
        data: Some("base64encodeddata".to_string()),
        mime_type: Some("image/png".to_string()),
    };
    assert!(binary_content.data.is_some());
    assert!(binary_content.mime_type.is_some());
}

#[test]
fn test_mcp_message() {
    let message = McpMessage {
        role: "assistant".to_string(),
        content: McpContent {
            content_type: "text".to_string(),
            text: Some("I can help you with that.".to_string()),
            data: None,
            mime_type: None,
        },
    };

    assert_eq!(message.role, "assistant");
}

#[test]
fn test_mcp_prompt_messages() {
    let prompt_messages = McpPromptMessages {
        description: Some("A helpful prompt".to_string()),
        messages: vec![
            McpMessage {
                role: "system".to_string(),
                content: McpContent {
                    content_type: "text".to_string(),
                    text: Some("You are a helpful assistant.".to_string()),
                    data: None,
                    mime_type: None,
                },
            },
            McpMessage {
                role: "user".to_string(),
                content: McpContent {
                    content_type: "text".to_string(),
                    text: Some("Help me with code review.".to_string()),
                    data: None,
                    mime_type: None,
                },
            },
        ],
    };

    assert_eq!(prompt_messages.messages.len(), 2);
    assert!(prompt_messages.description.is_some());
}

#[test]
fn test_mcp_client_url_construction() {
    let client = McpClient::new("localhost:8080", "test-server");
    // The base_url is private, but we can verify the client was created
    // In a real test, we'd mock the HTTP client

    let sse_client = McpClient::new_sse("localhost:8082", "sse-server");
    // SSE client uses different path
}

#[test]
fn test_mcp_serialization() {
    let tool = McpTool {
        name: "test".to_string(),
        description: Some("Test tool".to_string()),
        input_schema: None,
    };

    let json = serde_json::to_string(&tool).expect("Failed to serialize");
    assert!(json.contains("\"name\":\"test\""));

    let parsed: McpTool = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(parsed.name, tool.name);
}
