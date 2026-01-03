//! LLM provider tests for wiki-rs integration

#[path = "../src/llm.rs"]
mod llm;

use llm::*;

#[test]
fn test_llm_provider_default() {
    let provider: LlmProvider = Default::default();
    assert!(matches!(provider, LlmProvider::OpenAI));
}

#[test]
fn test_openai_provider() {
    let config = providers::openai();

    assert_eq!(config.provider, LlmProvider::OpenAI);
    assert!(config.base_url.contains("openai.com"));
    assert_eq!(config.api_key_env, Some("OPENAI_API_KEY".to_string()));
    assert_eq!(config.default_model, "gpt-4o");
    assert!(!config.models.is_empty());

    // Check models have required fields
    for model in &config.models {
        assert!(!model.id.is_empty());
        assert!(!model.name.is_empty());
    }
}

#[test]
fn test_anthropic_provider() {
    let config = providers::anthropic();

    assert_eq!(config.provider, LlmProvider::Anthropic);
    assert!(config.base_url.contains("anthropic.com"));
    assert_eq!(config.api_key_env, Some("ANTHROPIC_API_KEY".to_string()));
    assert!(config.headers.contains_key("anthropic-version"));
    assert!(!config.models.is_empty());
}

#[test]
fn test_ollama_provider() {
    let config = providers::ollama();

    assert_eq!(config.provider, LlmProvider::Ollama);
    assert!(config.base_url.contains("localhost:11434"));
    assert!(config.api_key_env.is_none()); // Ollama doesn't need API key
    assert!(!config.models.is_empty());
}

#[test]
fn test_azure_provider() {
    let config = providers::azure(
        "https://my-resource.openai.azure.com",
        "my-deployment"
    );

    assert_eq!(config.provider, LlmProvider::Azure);
    assert!(config.base_url.contains("my-resource"));
    assert!(config.base_url.contains("my-deployment"));
    assert_eq!(config.api_key_env, Some("AZURE_OPENAI_API_KEY".to_string()));
    assert!(config.headers.contains_key("api-version"));
}

#[test]
fn test_model_info() {
    let model = ModelInfo {
        id: "gpt-4o".to_string(),
        name: "GPT-4o".to_string(),
        context_window: Some(128000),
        max_output_tokens: Some(4096),
        supports_vision: true,
        supports_functions: true,
    };

    assert_eq!(model.id, "gpt-4o");
    assert!(model.supports_vision);
    assert!(model.supports_functions);
    assert_eq!(model.context_window, Some(128000));
}

#[test]
fn test_model_info_minimal() {
    let model = ModelInfo {
        id: "custom-model".to_string(),
        name: "Custom Model".to_string(),
        context_window: None,
        max_output_tokens: None,
        supports_vision: false,
        supports_functions: false,
    };

    assert!(model.context_window.is_none());
    assert!(!model.supports_vision);
}

#[test]
fn test_request_config_default() {
    let config = RequestConfig::default();

    assert_eq!(config.temperature, 0.7);
    assert_eq!(config.top_p, 1.0);
    assert_eq!(config.max_tokens, 4096);
    assert!(config.stop.is_empty());
    assert_eq!(config.presence_penalty, 0.0);
    assert_eq!(config.frequency_penalty, 0.0);
}

#[test]
fn test_request_config_custom() {
    let config = RequestConfig {
        temperature: 0.3,
        top_p: 0.9,
        max_tokens: 2048,
        stop: vec!["###".to_string(), "END".to_string()],
        presence_penalty: 0.5,
        frequency_penalty: 0.2,
    };

    assert_eq!(config.temperature, 0.3);
    assert_eq!(config.stop.len(), 2);
    assert!(config.stop.contains(&"###".to_string()));
}

#[test]
fn test_provider_config_serialization() {
    let config = providers::openai();

    let json = serde_json::to_string(&config).expect("Failed to serialize");
    assert!(json.contains("openai"));
    assert!(json.contains("gpt-4o"));

    let parsed: ProviderConfig = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(parsed.provider, config.provider);
    assert_eq!(parsed.default_model, config.default_model);
}

#[test]
fn test_llm_provider_serialization() {
    let providers = vec![
        (LlmProvider::OpenAI, "openai"),
        (LlmProvider::Anthropic, "anthropic"),
        (LlmProvider::Azure, "azure"),
        (LlmProvider::Ollama, "ollama"),
        (LlmProvider::Custom, "custom"),
    ];

    for (provider, expected) in providers {
        let json = serde_json::to_string(&provider).expect("Failed to serialize");
        assert_eq!(json, format!("\"{}\"", expected));
    }
}

#[test]
fn test_custom_headers() {
    let mut config = providers::openai();
    config.headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());

    assert!(config.headers.contains_key("X-Custom-Header"));
    assert_eq!(config.headers.get("X-Custom-Header"), Some(&"custom-value".to_string()));
}

#[test]
fn test_provider_models_have_context() {
    let openai = providers::openai();

    for model in &openai.models {
        // All OpenAI models should have context window info
        assert!(model.context_window.is_some());
    }
}
