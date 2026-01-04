//! Type-safe configuration loader for AgentAsKit
//!
//! This module provides unified configuration management across all AgentAsKit systems
//! with support for multiple formats (YAML, TOML, JSON), environment overrides, and validation.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration source precedence (highest to lowest):
/// 1. Environment variables (AGENTASKIT_*)
/// 2. .env file
/// 3. Config files (YAML/TOML/JSON)
/// 4. Defaults

/// Main configuration structure for AgentAsKit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAsKitConfig {
    /// Application metadata
    pub app: AppConfig,

    /// Tracing and observability configuration
    pub tracing: Option<TracingConfig>,

    /// Rate limiting configuration
    pub rate_limits: Option<RateLimitsConfig>,

    /// Server configuration
    pub server: Option<ServerConfig>,

    /// MCP/A2A gateway configuration
    pub gateway: Option<GatewayConfig>,

    /// Additional custom configurations
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Application metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: String, // dev, staging, production
    #[serde(default)]
    pub debug: bool,
}

/// Tracing and observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enable tracing
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Tracing level: trace, debug, info, warn, error
    #[serde(default)]
    pub level: String,

    /// OTLP exporter configuration
    pub otlp: Option<OtlpConfig>,

    /// Sample rate (0.0 to 1.0)
    #[serde(default)]
    pub sample_rate: f64,
}

/// OTLP (OpenTelemetry Protocol) exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtlpConfig {
    pub endpoint: String,
    #[serde(default)]
    pub timeout_seconds: u64,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitsConfig {
    /// Default requests per second
    pub default_rps: f64,

    /// Burst size (requests per burst)
    pub burst_size: u32,

    /// Per-endpoint rate limit overrides
    #[serde(default)]
    pub endpoints: HashMap<String, EndpointRateLimit>,
}

/// Per-endpoint rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointRateLimit {
    pub rps: f64,
    pub burst_size: u32,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,

    /// Number of worker threads
    #[serde(default)]
    pub workers: usize,

    /// Request timeout in seconds
    #[serde(default)]
    pub request_timeout: u64,

    /// TLS configuration
    pub tls: Option<TlsConfig>,
}

/// TLS/HTTPS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}

/// MCP/A2A gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// MCP protocol enabled
    #[serde(default)]
    pub mcp_enabled: bool,

    /// A2A protocol enabled
    #[serde(default)]
    pub a2a_enabled: bool,

    /// Gateway port
    #[serde(default)]
    pub port: u16,

    /// Tool registry
    pub tools: Option<ToolsConfig>,
}

/// Tools/integrations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    /// Tool definitions
    pub definitions: Vec<ToolDefinition>,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

impl Default for AgentAsKitConfig {
    fn default() -> Self {
        Self {
            app: AppConfig {
                name: "agentaskit".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                environment: env::var("AGENTASKIT_ENV").unwrap_or_else(|_| "dev".to_string()),
                debug: false,
            },
            tracing: None,
            rate_limits: None,
            server: None,
            gateway: None,
            extra: HashMap::new(),
        }
    }
}

/// Configuration loader with support for multiple formats and environment overrides
pub struct ConfigLoader {
    base_path: PathBuf,
    environment: String,
}

impl ConfigLoader {
    /// Create a new configuration loader
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        let environment = env::var("AGENTASKIT_ENV").unwrap_or_else(|_| "dev".to_string());

        Self {
            base_path: base_path.as_ref().to_path_buf(),
            environment,
        }
    }

    /// Load configuration from default locations
    pub fn load(&self) -> Result<AgentAsKitConfig> {
        // Try to load from environment-specific config first
        if let Ok(config) = self.load_from_env_specific() {
            return Ok(self.apply_env_overrides(config));
        }

        // Fall back to base config
        if let Ok(config) = self.load_from_file(&self.base_path.join("config.yaml")) {
            return Ok(self.apply_env_overrides(config));
        }

        // Use defaults
        Ok(self.apply_env_overrides(AgentAsKitConfig::default()))
    }

    /// Load configuration from environment-specific file
    fn load_from_env_specific(&self) -> Result<AgentAsKitConfig> {
        let config_file = self
            .base_path
            .join(format!("config-{}.yaml", self.environment));

        self.load_from_file(&config_file)
    }

    /// Load configuration from a specific file
    fn load_from_file(&self, path: &Path) -> Result<AgentAsKitConfig> {
        if !path.exists() {
            return Err(anyhow!("Config file not found: {:?}", path));
        }

        let content = fs::read_to_string(path).context(format!("Failed to read config: {:?}", path))?;

        // Detect format by extension
        let config = match path.extension().and_then(|ext| ext.to_str()) {
            Some("yaml") | Some("yml") => {
                serde_yaml::from_str(&content).context("Failed to parse YAML config")?
            }
            Some("toml") => {
                toml::from_str(&content).context("Failed to parse TOML config")?
            }
            Some("json") => {
                serde_json::from_str(&content).context("Failed to parse JSON config")?
            }
            _ => return Err(anyhow!("Unsupported config format")),
        };

        Ok(config)
    }

    /// Apply environment variable overrides to configuration
    fn apply_env_overrides(&self, mut config: AgentAsKitConfig) -> AgentAsKitConfig {
        // Application config overrides
        if let Ok(name) = env::var("AGENTASKIT_APP_NAME") {
            config.app.name = name;
        }
        if let Ok(version) = env::var("AGENTASKIT_APP_VERSION") {
            config.app.version = version;
        }
        if let Ok(env_str) = env::var("AGENTASKIT_ENV") {
            config.app.environment = env_str;
        }
        if let Ok(debug_str) = env::var("AGENTASKIT_DEBUG") {
            config.app.debug = debug_str.to_lowercase() == "true";
        }

        // Tracing overrides
        if let Ok(level) = env::var("AGENTASKIT_TRACE_LEVEL") {
            if let Some(tracing) = &mut config.tracing {
                tracing.level = level;
            } else {
                config.tracing = Some(TracingConfig {
                    enabled: true,
                    level,
                    otlp: None,
                    sample_rate: 1.0,
                });
            }
        }

        // Server overrides
        if let Ok(port_str) = env::var("AGENTASKIT_SERVER_PORT") {
            if let Ok(port) = port_str.parse() {
                if let Some(server) = &mut config.server {
                    server.port = port;
                } else {
                    config.server = Some(ServerConfig {
                        host: "127.0.0.1".to_string(),
                        port,
                        workers: num_cpus::get(),
                        request_timeout: 30,
                        tls: None,
                    });
                }
            }
        }

        config
    }

    /// Validate configuration
    pub fn validate(&self, config: &AgentAsKitConfig) -> Result<()> {
        // Validate app config
        if config.app.name.is_empty() {
            return Err(anyhow!("App name cannot be empty"));
        }

        // Validate tracing config if present
        if let Some(tracing) = &config.tracing {
            if !["trace", "debug", "info", "warn", "error"].contains(&tracing.level.as_str()) {
                return Err(anyhow!(
                    "Invalid tracing level: {}. Must be one of: trace, debug, info, warn, error",
                    tracing.level
                ));
            }

            if !(0.0..=1.0).contains(&tracing.sample_rate) {
                return Err(anyhow!(
                    "Invalid sample rate: {}. Must be between 0.0 and 1.0",
                    tracing.sample_rate
                ));
            }
        }

        // Validate rate limits if present
        if let Some(limits) = &config.rate_limits {
            if limits.default_rps <= 0.0 {
                return Err(anyhow!(
                    "Invalid default RPS: {}. Must be greater than 0",
                    limits.default_rps
                ));
            }
            if limits.burst_size == 0 {
                return Err(anyhow!("Invalid burst size: 0. Must be greater than 0"));
            }
        }

        // Validate server config if present
        if let Some(server) = &config.server {
            if server.port == 0 {
                return Err(anyhow!("Invalid server port: 0. Must be between 1 and 65535"));
            }
            if server.workers == 0 {
                return Err(anyhow!("Invalid worker count: 0. Must be at least 1"));
            }
        }

        Ok(())
    }
}

/// Builder for creating configurations programmatically
pub struct ConfigBuilder {
    config: AgentAsKitConfig,
}

impl ConfigBuilder {
    /// Create a new configuration builder with defaults
    pub fn new() -> Self {
        Self {
            config: AgentAsKitConfig::default(),
        }
    }

    /// Set application name
    pub fn app_name(mut self, name: impl Into<String>) -> Self {
        self.config.app.name = name.into();
        self
    }

    /// Set environment (dev, staging, production)
    pub fn environment(mut self, env: impl Into<String>) -> Self {
        self.config.app.environment = env.into();
        self
    }

    /// Set debug mode
    pub fn debug(mut self, debug: bool) -> Self {
        self.config.app.debug = debug;
        self
    }

    /// Set tracing configuration
    pub fn tracing(mut self, tracing: TracingConfig) -> Self {
        self.config.tracing = Some(tracing);
        self
    }

    /// Set rate limits configuration
    pub fn rate_limits(mut self, limits: RateLimitsConfig) -> Self {
        self.config.rate_limits = Some(limits);
        self
    }

    /// Set server configuration
    pub fn server(mut self, server: ServerConfig) -> Self {
        self.config.server = Some(server);
        self
    }

    /// Set gateway configuration
    pub fn gateway(mut self, gateway: GatewayConfig) -> Self {
        self.config.gateway = Some(gateway);
        self
    }

    /// Build the configuration
    pub fn build(self) -> AgentAsKitConfig {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions
fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AgentAsKitConfig::default();
        assert_eq!(config.app.name, "agentaskit");
        assert!(!config.app.debug);
    }

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .app_name("test-app")
            .environment("staging")
            .debug(true)
            .build();

        assert_eq!(config.app.name, "test-app");
        assert_eq!(config.app.environment, "staging");
        assert!(config.app.debug);
    }

    #[test]
    fn test_config_validation() {
        let loader = ConfigLoader::new(".");
        let mut config = AgentAsKitConfig::default();

        // Valid config should pass
        assert!(loader.validate(&config).is_ok());

        // Invalid tracing level should fail
        config.tracing = Some(TracingConfig {
            enabled: true,
            level: "invalid".to_string(),
            otlp: None,
            sample_rate: 1.0,
        });
        assert!(loader.validate(&config).is_err());
    }
}
