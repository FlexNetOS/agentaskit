//! Unified AI Provider abstraction using airust
//!
//! This module provides a unified interface to multiple AI providers through:
//! - airust: Rust-native async client library
//! - aichat: CLI integration for 20+ providers
//! - agentgateway: MCP-based tool routing

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Supported AI providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    OpenAI,
    Anthropic,
    Google,
    Bedrock,
    Cohere,
    Groq,
    Mistral,
    DeepSeek,
    Local,
    Custom,
}

impl Provider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::OpenAI => "openai",
            Provider::Anthropic => "claude",
            Provider::Google => "gemini",
            Provider::Bedrock => "bedrock",
            Provider::Cohere => "cohere",
            Provider::Groq => "groq",
            Provider::Mistral => "mistral",
            Provider::DeepSeek => "deepseek",
            Provider::Local => "local",
            Provider::Custom => "custom",
        }
    }
}

/// Message role in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// A chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
            name: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
            name: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
            name: None,
        }
    }
}

/// Completion request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

impl CompletionRequest {
    pub fn new(messages: Vec<ChatMessage>) -> Self {
        Self {
            messages,
            model: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            stop: None,
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

/// Completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub content: String,
    pub model: String,
    pub provider: Provider,
    pub usage: Option<Usage>,
    pub finish_reason: Option<String>,
}

/// Token usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Trait for AI providers
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Get the provider type
    fn provider(&self) -> Provider;

    /// Complete a chat request
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;

    /// Check if the provider is available
    async fn is_available(&self) -> bool;

    /// List available models
    async fn list_models(&self) -> Result<Vec<String>>;
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: Provider,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub default_model: Option<String>,
    pub timeout_secs: Option<u64>,
}

/// Unified provider manager
pub struct ProviderManager {
    providers: RwLock<HashMap<Provider, Arc<dyn AIProvider>>>,
    default_provider: RwLock<Option<Provider>>,
}

impl ProviderManager {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            default_provider: RwLock::new(None),
        }
    }

    /// Register a provider
    pub async fn register(&self, provider: Arc<dyn AIProvider>) {
        let provider_type = provider.provider();
        self.providers.write().await.insert(provider_type, provider);
    }

    /// Set the default provider
    pub async fn set_default(&self, provider: Provider) {
        *self.default_provider.write().await = Some(provider);
    }

    /// Get a provider by type
    pub async fn get(&self, provider: Provider) -> Option<Arc<dyn AIProvider>> {
        self.providers.read().await.get(&provider).cloned()
    }

    /// Get the default provider
    pub async fn get_default(&self) -> Option<Arc<dyn AIProvider>> {
        let default = self.default_provider.read().await;
        if let Some(provider_type) = *default {
            self.get(provider_type).await
        } else {
            // Return the first available provider
            self.providers.read().await.values().next().cloned()
        }
    }

    /// Complete using the default provider
    pub async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let provider = self
            .get_default()
            .await
            .ok_or_else(|| anyhow!("No AI provider configured"))?;
        provider.complete(request).await
    }

    /// List all registered providers
    pub async fn list_providers(&self) -> Vec<Provider> {
        self.providers.read().await.keys().copied().collect()
    }
}

impl Default for ProviderManager {
    fn default() -> Self {
        Self::new()
    }
}

/// AichatProvider - Uses aichat CLI as backend
pub struct AichatProvider {
    binary_path: String,
    default_model: Option<String>,
}

impl AichatProvider {
    pub fn new(binary_path: impl Into<String>) -> Self {
        Self {
            binary_path: binary_path.into(),
            default_model: None,
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = Some(model.into());
        self
    }
}

#[async_trait]
impl AIProvider for AichatProvider {
    fn provider(&self) -> Provider {
        Provider::Custom // aichat supports multiple providers
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        use tokio::process::Command;

        // Build the prompt from messages
        let prompt = request
            .messages
            .iter()
            .map(|m| match m.role {
                Role::System => format!("System: {}", m.content),
                Role::User => format!("User: {}", m.content),
                Role::Assistant => format!("Assistant: {}", m.content),
                Role::Tool => format!("Tool: {}", m.content),
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut cmd = Command::new(&self.binary_path);

        if let Some(model) = request.model.as_ref().or(self.default_model.as_ref()) {
            cmd.arg("-m").arg(model);
        }

        cmd.arg(&prompt);

        let output = cmd.output().await?;

        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(CompletionResponse {
                content,
                model: request.model.unwrap_or_else(|| "default".to_string()),
                provider: Provider::Custom,
                usage: None,
                finish_reason: Some("stop".to_string()),
            })
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("aichat error: {}", error))
        }
    }

    async fn is_available(&self) -> bool {
        std::path::Path::new(&self.binary_path).exists()
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        use tokio::process::Command;

        let output = Command::new(&self.binary_path)
            .arg("--list-models")
            .output()
            .await?;

        if output.status.success() {
            let models = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|s| s.to_string())
                .collect();
            Ok(models)
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_creation() {
        let msg = ChatMessage::user("Hello");
        assert!(matches!(msg.role, Role::User));
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_completion_request_builder() {
        let request = CompletionRequest::new(vec![ChatMessage::user("Test")])
            .with_model("gpt-4")
            .with_max_tokens(100)
            .with_temperature(0.7);

        assert_eq!(request.model, Some("gpt-4".to_string()));
        assert_eq!(request.max_tokens, Some(100));
        assert_eq!(request.temperature, Some(0.7));
    }

    #[tokio::test]
    async fn test_provider_manager() {
        let manager = ProviderManager::new();
        assert!(manager.list_providers().await.is_empty());
    }
}
