//! Gateway configuration module

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Gateway binary path (defaults to PATH lookup)
    #[serde(default)]
    pub binary_path: Option<PathBuf>,

    /// Configuration file path
    #[serde(default)]
    pub config_file: Option<PathBuf>,

    /// Static configuration
    #[serde(default)]
    pub static_config: StaticConfig,

    /// Listener configurations
    #[serde(default)]
    pub listeners: Vec<ListenerConfig>,

    /// Target (backend) configurations
    #[serde(default)]
    pub targets: Vec<TargetConfig>,

    /// MCP authentication settings
    #[serde(default)]
    pub mcp_authentication: Option<McpAuthConfig>,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            binary_path: None,
            config_file: None,
            static_config: StaticConfig::default(),
            listeners: vec![ListenerConfig::default()],
            targets: Vec::new(),
            mcp_authentication: None,
        }
    }
}

/// Static configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticConfig {
    /// Admin interface address
    #[serde(default = "default_admin_address")]
    pub admin_address: String,

    /// Log level
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Enable OpenTelemetry
    #[serde(default)]
    pub enable_telemetry: bool,

    /// OTLP endpoint for telemetry
    #[serde(default)]
    pub otlp_endpoint: Option<String>,
}

fn default_admin_address() -> String {
    "127.0.0.1:15000".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for StaticConfig {
    fn default() -> Self {
        Self {
            admin_address: default_admin_address(),
            log_level: default_log_level(),
            enable_telemetry: false,
            otlp_endpoint: None,
        }
    }
}

/// Listener configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerConfig {
    /// Listener name
    pub name: String,

    /// Protocol type: mcp, a2a, http
    #[serde(default = "default_protocol")]
    pub protocol: String,

    /// Listen address
    #[serde(default = "default_listen_address")]
    pub address: String,

    /// TLS configuration
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

fn default_protocol() -> String {
    "mcp".to_string()
}

fn default_listen_address() -> String {
    "127.0.0.1:8080".to_string()
}

impl Default for ListenerConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            protocol: default_protocol(),
            address: default_listen_address(),
            tls: None,
        }
    }
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file path
    pub cert_file: PathBuf,

    /// Key file path
    pub key_file: PathBuf,

    /// CA certificate file (for mTLS)
    #[serde(default)]
    pub ca_file: Option<PathBuf>,
}

/// Target (backend) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    /// Target name
    pub name: String,

    /// Target type: stdio, sse, http
    #[serde(rename = "type")]
    pub target_type: String,

    /// Host for HTTP/SSE targets
    #[serde(default)]
    pub host: Option<String>,

    /// Command for stdio targets
    #[serde(default)]
    pub command: Option<String>,

    /// Arguments for stdio targets
    #[serde(default)]
    pub args: Vec<String>,
}

/// MCP Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpAuthConfig {
    /// OAuth2 issuer URL
    pub issuer: String,

    /// JWKS URL for token validation
    #[serde(default)]
    pub jwks_url: Option<String>,

    /// Resource metadata configuration
    #[serde(default)]
    pub resource_metadata: Option<ResourceMetadata>,

    /// Provider-specific settings
    #[serde(default)]
    pub provider: Option<AuthProvider>,
}

/// Resource metadata for MCP authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    /// Resource identifier
    pub resource: String,

    /// Supported scopes
    #[serde(default)]
    pub scopes_supported: Vec<String>,

    /// Supported bearer methods
    #[serde(default)]
    pub bearer_methods_supported: Vec<String>,
}

/// Authentication provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthProvider {
    /// Keycloak provider
    Keycloak {},
    /// Generic OAuth2 provider
    Generic {},
}

impl GatewayConfig {
    /// Load configuration from a YAML file
    pub fn from_file(path: &PathBuf) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::Error::ConfigError(e.to_string()))?;
        serde_yaml::from_str(&content)
            .map_err(|e| crate::Error::ConfigError(e.to_string()))
    }

    /// Write configuration to a YAML file
    pub fn to_file(&self, path: &PathBuf) -> crate::Result<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| crate::Error::ConfigError(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Generate agentgateway-compatible configuration
    pub fn to_gateway_config(&self) -> serde_json::Value {
        let mut config = serde_json::json!({
            "version": "v1",
            "listeners": self.listeners.iter().map(|l| {
                serde_json::json!({
                    "name": l.name,
                    "protocol": l.protocol,
                    "address": l.address,
                })
            }).collect::<Vec<_>>(),
            "targets": self.targets.iter().map(|t| {
                let mut target = serde_json::json!({
                    "name": t.name,
                    "type": t.target_type,
                });
                if let Some(host) = &t.host {
                    target["host"] = serde_json::json!(host);
                }
                if let Some(cmd) = &t.command {
                    target["command"] = serde_json::json!(cmd);
                }
                if !t.args.is_empty() {
                    target["args"] = serde_json::json!(t.args);
                }
                target
            }).collect::<Vec<_>>(),
        });

        if let Some(auth) = &self.mcp_authentication {
            config["mcpAuthentication"] = serde_json::json!({
                "issuer": auth.issuer,
            });
            if let Some(jwks) = &auth.jwks_url {
                config["mcpAuthentication"]["jwksUrl"] = serde_json::json!(jwks);
            }
        }

        config
    }
}
