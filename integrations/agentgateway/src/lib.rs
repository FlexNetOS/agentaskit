//! AgentasKit Gateway Integration
//!
//! This crate provides integration between AgentasKit and Agentgateway,
//! enabling MCP, A2A, and other protocol support for agent communication.

pub mod config;
pub mod gateway;
pub mod mcp;
pub mod a2a;
pub mod auth;
pub mod observability;
pub mod routing;
pub mod ratelimit;
pub mod xds;

pub use config::GatewayConfig;
pub use gateway::GatewayManager;

/// Integration result type
pub type Result<T> = std::result::Result<T, Error>;

/// Integration error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Gateway not found: {0}")]
    GatewayNotFound(String),

    #[error("Gateway failed to start: {0}")]
    GatewayStartFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
