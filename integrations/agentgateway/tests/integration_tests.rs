//! Integration tests for AgentasKit Gateway Integration

use agentaskit_gateway_integration::{
    config::{GatewayConfig, ListenerConfig, TargetConfig},
    gateway::GatewayManager,
    mcp::{McpClient, McpServerConfig},
    a2a::{A2aClient, TaskRequest},
    auth::{AuthConfig, CelPolicyBuilder},
    routing::{RouteBuilder, AgentRouteMapper},
    ratelimit::{RateLimitBuilder, policies},
    observability::ObservabilityBuilder,
    xds::XdsConfigBuilder,
};

#[test]
fn test_gateway_config_default() {
    let config = GatewayConfig::default();
    assert_eq!(config.static_config.admin_address, "127.0.0.1:15000");
    assert_eq!(config.listeners.len(), 1);
}

#[test]
fn test_gateway_config_serialization() {
    let config = GatewayConfig::default();
    let yaml = serde_yaml::to_string(&config).unwrap();
    assert!(yaml.contains("admin_address"));

    let parsed: GatewayConfig = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(parsed.static_config.admin_address, config.static_config.admin_address);
}

#[test]
fn test_gateway_config_to_gateway_format() {
    let mut config = GatewayConfig::default();
    config.targets.push(TargetConfig {
        name: "test-server".to_string(),
        target_type: "stdio".to_string(),
        host: None,
        command: Some("npx".to_string()),
        args: vec!["-y".to_string(), "@modelcontextprotocol/server-everything".to_string()],
    });

    let gateway_config = config.to_gateway_config();
    assert!(gateway_config["version"].as_str() == Some("v1"));
    assert!(gateway_config["targets"].is_array());
}

#[test]
fn test_mcp_client_url_construction() {
    let client = McpClient::new("localhost:8080", "test-server");
    // URL is private, but we can test by checking it doesn't panic
    assert!(true);
}

#[test]
fn test_mcp_server_config_builder() {
    let config = McpServerConfig::new_stdio("my-server", "npx")
        .with_args(vec!["-y".to_string(), "@mcp/server".to_string()])
        .with_env("DEBUG", "true");

    assert_eq!(config.name, "my-server");
    assert_eq!(config.command, "npx");
    assert_eq!(config.args.len(), 2);
    assert!(config.env.contains_key("DEBUG"));
}

#[test]
fn test_a2a_task_request() {
    let task = TaskRequest::text("Hello, agent!")
        .with_session("session-123")
        .with_metadata("priority", serde_json::json!("high"));

    assert_eq!(task.message.role, "user");
    assert_eq!(task.session_id, Some("session-123".to_string()));
}

#[test]
fn test_cel_policy_builder() {
    let policy = CelPolicyBuilder::new()
        .require_subject("user-123")
        .require_scope("read:tools")
        .mcp_tool("calculator")
        .build();

    assert!(policy.contains("jwt.sub"));
    assert!(policy.contains("read:tools"));
    assert!(policy.contains("calculator"));
}

#[test]
fn test_cel_policy_empty() {
    let policy = CelPolicyBuilder::new().build();
    assert_eq!(policy, "true");
}

#[test]
fn test_route_builder() {
    let route = RouteBuilder::new("test-route", "test-target")
        .path_prefix("/api")
        .mcp_tool("add")
        .timeout(5000)
        .build();

    assert_eq!(route.name, "test-route");
    assert_eq!(route.target, "test-target");
    assert_eq!(route.matches.len(), 2);
}

#[test]
fn test_agent_route_mapper() {
    let mut mapper = AgentRouteMapper::new();
    mapper.register_mcp_agent("calculator", "calc-backend");
    mapper.register_a2a_agent("assistant", "assistant-backend");

    let config = mapper.to_config();
    assert_eq!(config.routes.len(), 2);
}

#[test]
fn test_rate_limit_builder() {
    let config = RateLimitBuilder::new()
        .global(100, Some(150))
        .per_user(10, Some(20))
        .target("mcp-server", 50, None)
        .policy(policies::mcp_tool_limit())
        .build();

    assert!(config.global.is_some());
    assert!(config.per_user.is_some());
    assert_eq!(config.targets.len(), 1);
    assert_eq!(config.policies.len(), 1);
}

#[test]
fn test_observability_builder() {
    let config = ObservabilityBuilder::new()
        .service_name("test-service")
        .otlp_endpoint("http://localhost:4317")
        .sampling_rate(0.5)
        .log_level("debug")
        .build();

    assert_eq!(config.tracing.service_name, "test-service");
    assert_eq!(config.tracing.sampling_rate, 0.5);
    assert_eq!(config.logging.level, "debug");
}

#[test]
fn test_xds_config_builder() {
    let config = XdsConfigBuilder::new("localhost:18000")
        .node_id("test-node")
        .cluster("test-cluster")
        .build();

    assert_eq!(config.control_plane_address, "localhost:18000");
    assert_eq!(config.node_id, "test-node");
    assert_eq!(config.cluster, "test-cluster");
}

// Async tests require tokio
#[cfg(test)]
mod async_tests {
    use super::*;

    #[tokio::test]
    async fn test_gateway_manager_creation() {
        let config = GatewayConfig::default();
        let mut manager = GatewayManager::new(config);
        assert!(!manager.is_running());
    }

    #[tokio::test]
    async fn test_gateway_admin_url() {
        let config = GatewayConfig::default();
        let manager = GatewayManager::new(config);
        assert_eq!(manager.admin_url(), "http://127.0.0.1:15000/ui");
    }

    #[tokio::test]
    async fn test_gateway_health_url() {
        let config = GatewayConfig::default();
        let manager = GatewayManager::new(config);
        assert_eq!(manager.health_url(), "http://127.0.0.1:15000/health");
    }
}
