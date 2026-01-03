//! LLM provider configuration for wiki generation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported LLM providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    OpenAI,
    Anthropic,
    Azure,
    Ollama,
    Custom,
}

impl Default for LlmProvider {
    fn default() -> Self {
        Self::OpenAI
    }
}

/// LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider type
    pub provider: LlmProvider,

    /// API base URL
    pub base_url: String,

    /// API key environment variable name
    #[serde(default)]
    pub api_key_env: Option<String>,

    /// Default model for this provider
    pub default_model: String,

    /// Available models
    #[serde(default)]
    pub models: Vec<ModelInfo>,

    /// Custom headers
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID
    pub id: String,

    /// Model display name
    pub name: String,

    /// Context window size
    #[serde(default)]
    pub context_window: Option<u32>,

    /// Maximum output tokens
    #[serde(default)]
    pub max_output_tokens: Option<u32>,

    /// Is this a vision model
    #[serde(default)]
    pub supports_vision: bool,

    /// Is this a function calling model
    #[serde(default)]
    pub supports_functions: bool,
}

/// Pre-configured providers
pub mod providers {
    use super::*;

    /// OpenAI configuration
    pub fn openai() -> ProviderConfig {
        ProviderConfig {
            provider: LlmProvider::OpenAI,
            base_url: "https://api.openai.com/v1".to_string(),
            api_key_env: Some("OPENAI_API_KEY".to_string()),
            default_model: "gpt-4o".to_string(),
            models: vec![
                ModelInfo {
                    id: "gpt-4o".to_string(),
                    name: "GPT-4o".to_string(),
                    context_window: Some(128000),
                    max_output_tokens: Some(4096),
                    supports_vision: true,
                    supports_functions: true,
                },
                ModelInfo {
                    id: "gpt-4o-mini".to_string(),
                    name: "GPT-4o Mini".to_string(),
                    context_window: Some(128000),
                    max_output_tokens: Some(4096),
                    supports_vision: true,
                    supports_functions: true,
                },
                ModelInfo {
                    id: "gpt-4-turbo".to_string(),
                    name: "GPT-4 Turbo".to_string(),
                    context_window: Some(128000),
                    max_output_tokens: Some(4096),
                    supports_vision: true,
                    supports_functions: true,
                },
            ],
            headers: HashMap::new(),
        }
    }

    /// Anthropic configuration
    pub fn anthropic() -> ProviderConfig {
        ProviderConfig {
            provider: LlmProvider::Anthropic,
            base_url: "https://api.anthropic.com/v1".to_string(),
            api_key_env: Some("ANTHROPIC_API_KEY".to_string()),
            default_model: "claude-sonnet-4-20250514".to_string(),
            models: vec![
                ModelInfo {
                    id: "claude-sonnet-4-20250514".to_string(),
                    name: "Claude Sonnet 4".to_string(),
                    context_window: Some(200000),
                    max_output_tokens: Some(8192),
                    supports_vision: true,
                    supports_functions: true,
                },
                ModelInfo {
                    id: "claude-opus-4-20250514".to_string(),
                    name: "Claude Opus 4".to_string(),
                    context_window: Some(200000),
                    max_output_tokens: Some(8192),
                    supports_vision: true,
                    supports_functions: true,
                },
            ],
            headers: {
                let mut h = HashMap::new();
                h.insert("anthropic-version".to_string(), "2023-06-01".to_string());
                h
            },
        }
    }

    /// Ollama local configuration
    pub fn ollama() -> ProviderConfig {
        ProviderConfig {
            provider: LlmProvider::Ollama,
            base_url: "http://localhost:11434/v1".to_string(),
            api_key_env: None,
            default_model: "llama3.2".to_string(),
            models: vec![
                ModelInfo {
                    id: "llama3.2".to_string(),
                    name: "Llama 3.2".to_string(),
                    context_window: Some(128000),
                    max_output_tokens: None,
                    supports_vision: false,
                    supports_functions: false,
                },
                ModelInfo {
                    id: "codellama".to_string(),
                    name: "Code Llama".to_string(),
                    context_window: Some(16000),
                    max_output_tokens: None,
                    supports_vision: false,
                    supports_functions: false,
                },
                ModelInfo {
                    id: "deepseek-coder".to_string(),
                    name: "DeepSeek Coder".to_string(),
                    context_window: Some(16000),
                    max_output_tokens: None,
                    supports_vision: false,
                    supports_functions: false,
                },
            ],
            headers: HashMap::new(),
        }
    }

    /// Azure OpenAI configuration
    pub fn azure(endpoint: &str, deployment: &str) -> ProviderConfig {
        ProviderConfig {
            provider: LlmProvider::Azure,
            base_url: format!("{}/openai/deployments/{}", endpoint, deployment),
            api_key_env: Some("AZURE_OPENAI_API_KEY".to_string()),
            default_model: deployment.to_string(),
            models: vec![],
            headers: {
                let mut h = HashMap::new();
                h.insert("api-version".to_string(), "2024-02-15-preview".to_string());
                h
            },
        }
    }
}

/// LLM request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestConfig {
    /// Temperature (0.0 - 2.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Top P sampling
    #[serde(default = "default_top_p")]
    pub top_p: f32,

    /// Maximum tokens in response
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,

    /// Stop sequences
    #[serde(default)]
    pub stop: Vec<String>,

    /// Presence penalty
    #[serde(default)]
    pub presence_penalty: f32,

    /// Frequency penalty
    #[serde(default)]
    pub frequency_penalty: f32,
}

fn default_temperature() -> f32 {
    0.7
}

fn default_top_p() -> f32 {
    1.0
}

fn default_max_tokens() -> u32 {
    4096
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            temperature: default_temperature(),
            top_p: default_top_p(),
            max_tokens: default_max_tokens(),
            stop: Vec::new(),
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_provider() {
        let config = providers::openai();
        assert_eq!(config.provider, LlmProvider::OpenAI);
        assert!(config.base_url.contains("openai.com"));
        assert!(!config.models.is_empty());
    }

    #[test]
    fn test_anthropic_provider() {
        let config = providers::anthropic();
        assert_eq!(config.provider, LlmProvider::Anthropic);
        assert!(config.headers.contains_key("anthropic-version"));
    }

    #[test]
    fn test_ollama_provider() {
        let config = providers::ollama();
        assert_eq!(config.provider, LlmProvider::Ollama);
        assert!(config.api_key_env.is_none());
    }

    #[test]
    fn test_request_config_default() {
        let config = RequestConfig::default();
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_tokens, 4096);
    }
}
