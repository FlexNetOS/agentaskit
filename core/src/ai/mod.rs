//! AI Integration Module
//!
//! Provides unified access to AI capabilities:
//! - Multiple provider support via airust and aichat
//! - Gateway bridge for MCP/A2A protocols
//! - Local inference via llama.cpp
//! - Model selection and routing

pub mod gateway_bridge;
pub mod model_selector_bridge;
pub mod provider;
pub mod sop_analyzer;

// Re-export commonly used types
pub use gateway_bridge::{
    create_gateway, AgentCard, AgentSkill, GatewayBridge, GatewayConfig, McpTool, SharedGateway,
    ToolCallRequest, ToolCallResponse,
};
pub use provider::{
    AIProvider, AichatProvider, ChatMessage, CompletionRequest, CompletionResponse, Provider,
    ProviderConfig, ProviderManager, Role, Usage,
};

/// Initialize the AI subsystem with default configuration
pub async fn init_default() -> anyhow::Result<(ProviderManager, SharedGateway)> {
    let provider_manager = ProviderManager::new();
    let gateway = create_gateway(GatewayConfig::default())?;

    // Try to register aichat provider if available
    let aichat_paths = [
        "./integrations/aichat/target/release/aichat",
        "./tools/bin/aichat",
        "aichat", // System PATH
    ];

    for path in aichat_paths {
        if std::path::Path::new(path).exists() || which::which(path).is_ok() {
            let aichat = std::sync::Arc::new(AichatProvider::new(path));
            provider_manager.register(aichat).await;
            break;
        }
    }

    Ok((provider_manager, gateway))
}

/// Quick completion using default provider
pub async fn complete(prompt: &str) -> anyhow::Result<String> {
    let (manager, _) = init_default().await?;

    let request = CompletionRequest::new(vec![ChatMessage::user(prompt)]);

    let response = manager.complete(request).await?;
    Ok(response.content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_default() {
        // This will work even without aichat installed
        let result = init_default().await;
        assert!(result.is_ok());
    }
}
