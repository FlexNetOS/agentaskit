//! Configuration tests for wiki-rs integration

use std::path::PathBuf;

#[path = "../src/config.rs"]
mod config;

use config::*;

#[test]
fn test_wiki_config_default() {
    let config = WikiConfig::default();

    assert_eq!(config.project_path, PathBuf::from("."));
    assert_eq!(config.output_dir, PathBuf::from("./litho.docs"));
    assert_eq!(config.target_language, "en");
}

#[test]
fn test_wiki_config_for_project() {
    let config = WikiConfig::for_project(PathBuf::from("./my-project"))
        .with_output_dir(PathBuf::from("./docs"))
        .with_language("ja");

    assert_eq!(config.project_path, PathBuf::from("./my-project"));
    assert_eq!(config.output_dir, PathBuf::from("./docs"));
    assert_eq!(config.target_language, "ja");
}

#[test]
fn test_llm_config_default() {
    let config = LlmConfig::default();

    assert!(config.api_base_url.is_none());
    assert!(config.api_key.is_none());
    assert_eq!(config.model_efficient, "gpt-4o-mini");
    assert_eq!(config.model_powerful, "gpt-4o");
    assert!(config.enable_preset_tools);
    assert_eq!(config.max_tokens, 4096);
    assert_eq!(config.temperature, 0.7);
}

#[test]
fn test_processing_config_default() {
    let config = ProcessingConfig::default();

    assert!(!config.skip_preprocessing);
    assert!(!config.skip_research);
    assert!(config.enable_cache);
    assert_eq!(config.cache_dir, PathBuf::from("./.litho_cache"));
    assert!(!config.exclude_patterns.is_empty());
    assert_eq!(config.max_file_size, 1024 * 1024);
}

#[test]
fn test_exclude_patterns() {
    let config = ProcessingConfig::default();

    assert!(config.exclude_patterns.contains(&"node_modules/**".to_string()));
    assert!(config.exclude_patterns.contains(&"target/**".to_string()));
    assert!(config.exclude_patterns.contains(&".git/**".to_string()));
    assert!(config.exclude_patterns.contains(&"*.lock".to_string()));
}

#[test]
fn test_output_config_default() {
    let config = OutputConfig::default();

    assert!(config.generate_overview);
    assert!(config.generate_architecture);
    assert!(config.generate_workflow);
    assert!(config.generate_deep_dive);
    assert!(config.generate_diagrams);
    assert!(config.fix_mermaid_errors);
    assert!(matches!(config.format, OutputFormat::Markdown));
}

#[test]
fn test_output_format() {
    let formats = vec![
        (OutputFormat::Markdown, "markdown"),
        (OutputFormat::Html, "html"),
        (OutputFormat::Pdf, "pdf"),
    ];

    for (format, expected) in formats {
        let json = serde_json::to_string(&format).expect("Failed to serialize");
        assert_eq!(json, format!("\"{}\"", expected));
    }
}

#[test]
fn test_wiki_config_serialization() {
    let config = WikiConfig::default();

    // Serialize to TOML
    let toml = toml::to_string_pretty(&config).expect("Failed to serialize");
    assert!(toml.contains("project_path"));
    assert!(toml.contains("output_dir"));
    assert!(toml.contains("target_language"));

    // Check it can be deserialized
    let parsed: WikiConfig = toml::from_str(&toml).expect("Failed to deserialize");
    assert_eq!(parsed.target_language, config.target_language);
}

#[test]
fn test_wiki_config_with_llm() {
    let llm_config = LlmConfig {
        api_base_url: Some("https://api.custom.com/v1".to_string()),
        api_key: Some("test-key".to_string()),
        model_efficient: "custom-small".to_string(),
        model_powerful: "custom-large".to_string(),
        enable_preset_tools: false,
        max_tokens: 8192,
        temperature: 0.5,
    };

    let config = WikiConfig::default()
        .with_llm(llm_config);

    assert_eq!(config.llm.model_efficient, "custom-small");
    assert!(!config.llm.enable_preset_tools);
    assert_eq!(config.llm.max_tokens, 8192);
}

#[test]
fn test_languages() {
    let languages = vec!["en", "zh", "ja", "ko", "de", "fr", "es"];

    for lang in languages {
        let config = WikiConfig::default().with_language(lang);
        assert_eq!(config.target_language, lang);
    }
}
