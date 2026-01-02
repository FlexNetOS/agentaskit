//! Configuration tests for agentgateway integration

use std::path::PathBuf;

// Import from the parent crate
#[path = "../src/config.rs"]
mod config;

use config::*;

#[test]
fn test_gateway_config_default() {
    let config = GatewayConfig::default();

    assert_eq!(config.static_config.admin_address, "127.0.0.1:15000");
    assert_eq!(config.static_config.log_level, "info");
    assert!(!config.static_config.enable_telemetry);
    assert_eq!(config.listeners.len(), 1);
    assert!(config.targets.is_empty());
}

#[test]
fn test_listener_config_default() {
    let listener = ListenerConfig::default();

    assert_eq!(listener.name, "default");
    assert_eq!(listener.protocol, "mcp");
    assert_eq!(listener.address, "127.0.0.1:8080");
    assert!(listener.tls.is_none());
}

#[test]
fn test_target_config_stdio() {
    let target = TargetConfig {
        name: "test-server".to_string(),
        target_type: "stdio".to_string(),
        host: None,
        command: Some("npx".to_string()),
        args: vec!["-y".to_string(), "@mcp/server".to_string()],
    };

    assert_eq!(target.name, "test-server");
    assert_eq!(target.target_type, "stdio");
    assert!(target.command.is_some());
    assert_eq!(target.args.len(), 2);
}

#[test]
fn test_target_config_http() {
    let target = TargetConfig {
        name: "http-backend".to_string(),
        target_type: "http".to_string(),
        host: Some("http://localhost:3000".to_string()),
        command: None,
        args: vec![],
    };

    assert_eq!(target.target_type, "http");
    assert!(target.host.is_some());
    assert!(target.command.is_none());
}

#[test]
fn test_mcp_auth_config() {
    let auth = McpAuthConfig {
        issuer: "https://auth.example.com".to_string(),
        jwks_url: Some("https://auth.example.com/.well-known/jwks.json".to_string()),
        resource_metadata: Some(ResourceMetadata {
            resource: "http://localhost:8080/mcp".to_string(),
            scopes_supported: vec!["read:tools".to_string(), "write:tools".to_string()],
            bearer_methods_supported: vec!["header".to_string()],
        }),
        provider: None,
    };

    assert!(auth.jwks_url.is_some());
    assert!(auth.resource_metadata.is_some());

    let metadata = auth.resource_metadata.unwrap();
    assert_eq!(metadata.scopes_supported.len(), 2);
}

#[test]
fn test_tls_config() {
    let tls = TlsConfig {
        cert_file: PathBuf::from("/etc/certs/server.crt"),
        key_file: PathBuf::from("/etc/certs/server.key"),
        ca_file: Some(PathBuf::from("/etc/certs/ca.crt")),
    };

    assert!(tls.ca_file.is_some());
}

#[test]
fn test_gateway_config_serialization() {
    let config = GatewayConfig::default();

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&config).expect("Failed to serialize");
    assert!(yaml.contains("admin_address"));
    assert!(yaml.contains("127.0.0.1:15000"));

    // Deserialize back
    let parsed: GatewayConfig = serde_yaml::from_str(&yaml).expect("Failed to deserialize");
    assert_eq!(parsed.static_config.admin_address, config.static_config.admin_address);
}

#[test]
fn test_gateway_config_to_gateway_format() {
    let mut config = GatewayConfig::default();
    config.targets.push(TargetConfig {
        name: "test".to_string(),
        target_type: "stdio".to_string(),
        host: None,
        command: Some("echo".to_string()),
        args: vec!["hello".to_string()],
    });

    let gateway_config = config.to_gateway_config();

    assert_eq!(gateway_config["version"], "v1");
    assert!(gateway_config["listeners"].is_array());
    assert!(gateway_config["targets"].is_array());

    let targets = gateway_config["targets"].as_array().unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0]["name"], "test");
}

#[test]
fn test_static_config_defaults() {
    let static_config = StaticConfig::default();

    assert_eq!(static_config.admin_address, "127.0.0.1:15000");
    assert_eq!(static_config.log_level, "info");
    assert!(!static_config.enable_telemetry);
    assert!(static_config.otlp_endpoint.is_none());
}
