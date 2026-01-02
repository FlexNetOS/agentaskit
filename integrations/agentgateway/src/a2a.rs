//! Agent2Agent (A2A) protocol integration
//!
//! This module provides A2A client functionality for agent-to-agent
//! communication through agentgateway.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A2A client for inter-agent communication via gateway
pub struct A2aClient {
    base_url: String,
    client: reqwest::Client,
}

impl A2aClient {
    /// Create a new A2A client
    pub fn new(gateway_address: &str, agent_name: &str) -> Self {
        Self {
            base_url: format!("http://{}/a2a/{}", gateway_address, agent_name),
            client: reqwest::Client::new(),
        }
    }

    /// Get the agent card (capabilities and metadata)
    pub async fn get_agent_card(&self) -> Result<AgentCard> {
        let response = self.client
            .get(&format!("{}/.well-known/agent.json", self.base_url))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Send a task to the agent
    pub async fn send_task(&self, task: &TaskRequest) -> Result<TaskResponse> {
        let response = self.client
            .post(&format!("{}/tasks/send", self.base_url))
            .json(task)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Get task status
    pub async fn get_task(&self, task_id: &str) -> Result<TaskResponse> {
        let response = self.client
            .get(&format!("{}/tasks/{}", self.base_url, task_id))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Cancel a task
    pub async fn cancel_task(&self, task_id: &str) -> Result<TaskResponse> {
        let response = self.client
            .post(&format!("{}/tasks/{}/cancel", self.base_url, task_id))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Stream task updates via SSE
    pub async fn stream_task(&self, task_id: &str) -> Result<reqwest::Response> {
        let response = self.client
            .get(&format!("{}/tasks/{}/stream", self.base_url, task_id))
            .header("Accept", "text/event-stream")
            .send()
            .await?;

        Ok(response)
    }
}

/// A2A Agent Card - describes agent capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    /// Agent name
    pub name: String,

    /// Agent description
    #[serde(default)]
    pub description: Option<String>,

    /// Agent version
    #[serde(default)]
    pub version: Option<String>,

    /// URL for the agent
    #[serde(default)]
    pub url: Option<String>,

    /// Provider information
    #[serde(default)]
    pub provider: Option<AgentProvider>,

    /// Capabilities
    #[serde(default)]
    pub capabilities: AgentCapabilities,

    /// Skills the agent can perform
    #[serde(default)]
    pub skills: Vec<AgentSkill>,

    /// Authentication requirements
    #[serde(default)]
    pub authentication: Option<AuthenticationInfo>,
}

/// Agent provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProvider {
    pub organization: String,
    #[serde(default)]
    pub url: Option<String>,
}

/// Agent capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentCapabilities {
    /// Supports streaming responses
    #[serde(default)]
    pub streaming: bool,

    /// Supports push notifications
    #[serde(default)]
    pub push_notifications: bool,

    /// Supports state management
    #[serde(default)]
    pub state_transition_history: bool,
}

/// Agent skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkill {
    /// Skill ID
    pub id: String,

    /// Skill name
    pub name: String,

    /// Skill description
    #[serde(default)]
    pub description: Option<String>,

    /// Input schema
    #[serde(default)]
    pub input_schema: Option<serde_json::Value>,

    /// Output schema
    #[serde(default)]
    pub output_schema: Option<serde_json::Value>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Example inputs
    #[serde(default)]
    pub examples: Vec<SkillExample>,
}

/// Example input for a skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExample {
    pub name: String,
    pub input: serde_json::Value,
    #[serde(default)]
    pub output: Option<serde_json::Value>,
}

/// Authentication information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationInfo {
    #[serde(default)]
    pub schemes: Vec<String>,
}

/// Task request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequest {
    /// Task ID (optional, generated if not provided)
    #[serde(default)]
    pub id: Option<String>,

    /// Session ID for related tasks
    #[serde(default)]
    pub session_id: Option<String>,

    /// Message content
    pub message: TaskMessage,

    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,

    /// Push notification config
    #[serde(default)]
    pub push_notification: Option<PushNotificationConfig>,
}

/// Task message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMessage {
    /// Message role (user, assistant, system)
    pub role: String,

    /// Message parts
    pub parts: Vec<MessagePart>,
}

/// Message part
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessagePart {
    #[serde(rename = "text")]
    Text { text: String },

    #[serde(rename = "file")]
    File {
        #[serde(default)]
        file_id: Option<String>,
        #[serde(default)]
        file_name: Option<String>,
        #[serde(default)]
        mime_type: Option<String>,
        #[serde(default)]
        data: Option<String>,
    },

    #[serde(rename = "data")]
    Data {
        #[serde(default)]
        mime_type: Option<String>,
        data: String,
    },
}

/// Push notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotificationConfig {
    pub url: String,
    #[serde(default)]
    pub authentication: Option<PushAuthentication>,
}

/// Push notification authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushAuthentication {
    #[serde(default)]
    pub schemes: Vec<String>,
    #[serde(default)]
    pub credentials: Option<String>,
}

/// Task response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    /// Task ID
    pub id: String,

    /// Session ID
    #[serde(default)]
    pub session_id: Option<String>,

    /// Task status
    pub status: TaskStatus,

    /// Result artifacts
    #[serde(default)]
    pub artifacts: Vec<Artifact>,

    /// History of status changes
    #[serde(default)]
    pub history: Vec<TaskHistoryEntry>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    /// State of the task
    pub state: TaskState,

    /// Optional status message
    #[serde(default)]
    pub message: Option<TaskMessage>,

    /// Timestamp
    #[serde(default)]
    pub timestamp: Option<String>,
}

/// Task state enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskState {
    Submitted,
    Working,
    InputRequired,
    Completed,
    Canceled,
    Failed,
    Unknown,
}

/// Result artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// Artifact name
    #[serde(default)]
    pub name: Option<String>,

    /// Artifact description
    #[serde(default)]
    pub description: Option<String>,

    /// Artifact parts
    pub parts: Vec<MessagePart>,

    /// Index/order
    #[serde(default)]
    pub index: Option<u32>,

    /// Append to previous artifact
    #[serde(default)]
    pub append: bool,

    /// Last chunk indicator
    #[serde(default)]
    pub last_chunk: bool,
}

/// Task history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHistoryEntry {
    pub state: TaskState,
    #[serde(default)]
    pub message: Option<TaskMessage>,
    #[serde(default)]
    pub timestamp: Option<String>,
}

impl TaskRequest {
    /// Create a simple text task
    pub fn text(content: &str) -> Self {
        Self {
            id: None,
            session_id: None,
            message: TaskMessage {
                role: "user".to_string(),
                parts: vec![MessagePart::Text { text: content.to_string() }],
            },
            metadata: HashMap::new(),
            push_notification: None,
        }
    }

    /// Set session ID
    pub fn with_session(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a2a_client_new() {
        let client = A2aClient::new("localhost:8080", "test-agent");
        assert_eq!(client.base_url, "http://localhost:8080/a2a/test-agent");
    }

    #[test]
    fn test_task_request_text() {
        let task = TaskRequest::text("Hello, agent!")
            .with_session("session-123")
            .with_metadata("priority", serde_json::json!("high"));

        assert_eq!(task.session_id, Some("session-123".to_string()));
        assert!(task.metadata.contains_key("priority"));
    }

    #[test]
    fn test_task_state_serialization() {
        let state = TaskState::Working;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"working\"");
    }
}
