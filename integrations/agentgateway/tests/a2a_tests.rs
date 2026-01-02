//! A2A protocol tests for agentgateway integration

#[path = "../src/a2a.rs"]
mod a2a;

use a2a::*;

#[test]
fn test_agent_card() {
    let card = AgentCard {
        name: "TestAgent".to_string(),
        description: Some("A test agent".to_string()),
        version: Some("1.0.0".to_string()),
        url: Some("https://agent.example.com".to_string()),
        provider: Some(AgentProvider {
            organization: "Test Org".to_string(),
            url: Some("https://test.org".to_string()),
        }),
        capabilities: AgentCapabilities {
            streaming: true,
            push_notifications: false,
            state_transition_history: true,
        },
        skills: vec![
            AgentSkill {
                id: "code-review".to_string(),
                name: "Code Review".to_string(),
                description: Some("Reviews code for issues".to_string()),
                input_schema: None,
                output_schema: None,
                tags: vec!["development".to_string(), "quality".to_string()],
                examples: vec![],
            }
        ],
        authentication: None,
    };

    assert_eq!(card.name, "TestAgent");
    assert!(card.capabilities.streaming);
    assert!(!card.capabilities.push_notifications);
    assert_eq!(card.skills.len(), 1);
}

#[test]
fn test_task_request_text() {
    let task = TaskRequest::text("Hello, can you help me?")
        .with_session("session-123")
        .with_metadata("priority", serde_json::json!("high"))
        .with_metadata("user_id", serde_json::json!("user-456"));

    assert_eq!(task.message.role, "user");
    assert_eq!(task.session_id, Some("session-123".to_string()));
    assert_eq!(task.metadata.len(), 2);

    // Check message parts
    assert_eq!(task.message.parts.len(), 1);
    match &task.message.parts[0] {
        MessagePart::Text { text } => assert_eq!(text, "Hello, can you help me?"),
        _ => panic!("Expected Text part"),
    }
}

#[test]
fn test_task_state_serialization() {
    let states = vec![
        (TaskState::Submitted, "submitted"),
        (TaskState::Working, "working"),
        (TaskState::InputRequired, "input_required"),
        (TaskState::Completed, "completed"),
        (TaskState::Canceled, "canceled"),
        (TaskState::Failed, "failed"),
    ];

    for (state, expected) in states {
        let json = serde_json::to_string(&state).expect("Failed to serialize");
        assert_eq!(json, format!("\"{}\"", expected));
    }
}

#[test]
fn test_task_response() {
    let response = TaskResponse {
        id: "task-123".to_string(),
        session_id: Some("session-456".to_string()),
        status: TaskStatus {
            state: TaskState::Completed,
            message: None,
            timestamp: Some("2024-01-01T00:00:00Z".to_string()),
        },
        artifacts: vec![
            Artifact {
                name: Some("result".to_string()),
                description: Some("The task result".to_string()),
                parts: vec![
                    MessagePart::Text { text: "Task completed successfully".to_string() }
                ],
                index: Some(0),
                append: false,
                last_chunk: true,
            }
        ],
        history: vec![
            TaskHistoryEntry {
                state: TaskState::Submitted,
                message: None,
                timestamp: Some("2024-01-01T00:00:00Z".to_string()),
            },
            TaskHistoryEntry {
                state: TaskState::Working,
                message: None,
                timestamp: Some("2024-01-01T00:00:01Z".to_string()),
            },
        ],
        metadata: std::collections::HashMap::new(),
    };

    assert_eq!(response.id, "task-123");
    assert_eq!(response.status.state, TaskState::Completed);
    assert_eq!(response.artifacts.len(), 1);
    assert_eq!(response.history.len(), 2);
}

#[test]
fn test_message_parts() {
    // Text part
    let text_part = MessagePart::Text { text: "Hello".to_string() };
    let json = serde_json::to_string(&text_part).expect("Failed to serialize");
    assert!(json.contains("\"type\":\"text\""));

    // File part
    let file_part = MessagePart::File {
        file_id: Some("file-123".to_string()),
        file_name: Some("test.txt".to_string()),
        mime_type: Some("text/plain".to_string()),
        data: None,
    };
    let json = serde_json::to_string(&file_part).expect("Failed to serialize");
    assert!(json.contains("\"type\":\"file\""));

    // Data part
    let data_part = MessagePart::Data {
        mime_type: Some("application/json".to_string()),
        data: "{\"key\": \"value\"}".to_string(),
    };
    let json = serde_json::to_string(&data_part).expect("Failed to serialize");
    assert!(json.contains("\"type\":\"data\""));
}

#[test]
fn test_agent_skill() {
    let skill = AgentSkill {
        id: "summarize".to_string(),
        name: "Summarize Text".to_string(),
        description: Some("Summarizes long text into key points".to_string()),
        input_schema: Some(serde_json::json!({
            "type": "object",
            "properties": {
                "text": {"type": "string"},
                "max_length": {"type": "integer"}
            },
            "required": ["text"]
        })),
        output_schema: Some(serde_json::json!({
            "type": "object",
            "properties": {
                "summary": {"type": "string"},
                "key_points": {"type": "array", "items": {"type": "string"}}
            }
        })),
        tags: vec!["nlp".to_string(), "text".to_string()],
        examples: vec![
            SkillExample {
                name: "Basic summary".to_string(),
                input: serde_json::json!({"text": "Long text here..."}),
                output: Some(serde_json::json!({"summary": "Short summary"})),
            }
        ],
    };

    assert_eq!(skill.id, "summarize");
    assert!(skill.input_schema.is_some());
    assert_eq!(skill.tags.len(), 2);
    assert_eq!(skill.examples.len(), 1);
}

#[test]
fn test_push_notification_config() {
    let config = PushNotificationConfig {
        url: "https://webhook.example.com/notify".to_string(),
        authentication: Some(PushAuthentication {
            schemes: vec!["bearer".to_string()],
            credentials: Some("token-123".to_string()),
        }),
    };

    assert!(config.url.starts_with("https://"));
    assert!(config.authentication.is_some());
}

#[test]
fn test_agent_capabilities_default() {
    let caps = AgentCapabilities::default();

    assert!(!caps.streaming);
    assert!(!caps.push_notifications);
    assert!(!caps.state_transition_history);
}

#[test]
fn test_a2a_client_url() {
    let client = A2aClient::new("localhost:8081", "my-agent");
    // Verify client is created (URL is private)
}

#[test]
fn test_artifact() {
    let artifact = Artifact {
        name: Some("code".to_string()),
        description: Some("Generated code".to_string()),
        parts: vec![
            MessagePart::Text { text: "fn main() {}".to_string() }
        ],
        index: Some(0),
        append: false,
        last_chunk: true,
    };

    assert!(artifact.last_chunk);
    assert!(!artifact.append);
    assert_eq!(artifact.parts.len(), 1);
}
