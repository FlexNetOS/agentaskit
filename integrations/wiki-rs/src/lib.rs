//! AgentasKit Wiki Integration (Litho/deepwiki-rs)
//!
//! This crate provides integration with Litho, an AI-powered documentation
//! generator that automatically creates C4 architecture documentation from
//! source code.

pub mod config;
pub mod generator;
pub mod llm;
pub mod output;

pub use config::WikiConfig;
pub use generator::WikiGenerator;

/// Integration result type
pub type Result<T> = std::result::Result<T, Error>;

/// Integration error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Wiki generator not found: {0}")]
    GeneratorNotFound(String),

    #[error("Generation failed: {0}")]
    GenerationFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("LLM error: {0}")]
    LlmError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),
}
