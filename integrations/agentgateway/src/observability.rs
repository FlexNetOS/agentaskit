//! Observability integration for OpenTelemetry
//!
//! This module provides configuration and helpers for integrating
//! agentgateway telemetry with AgentasKit observability.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Enable tracing
    #[serde(default)]
    pub tracing: TracingConfig,

    /// Enable metrics
    #[serde(default)]
    pub metrics: MetricsConfig,

    /// Enable logging
    #[serde(default)]
    pub logging: LoggingConfig,

    /// Custom attributes to include
    #[serde(default)]
    pub custom_attributes: HashMap<String, String>,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            tracing: TracingConfig::default(),
            metrics: MetricsConfig::default(),
            logging: LoggingConfig::default(),
            custom_attributes: HashMap::new(),
        }
    }
}

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enable distributed tracing
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// OTLP endpoint for trace export
    #[serde(default)]
    pub otlp_endpoint: Option<String>,

    /// Service name for traces
    #[serde(default = "default_service_name")]
    pub service_name: String,

    /// Sampling rate (0.0 - 1.0)
    #[serde(default = "default_sampling_rate")]
    pub sampling_rate: f64,

    /// Propagation format: w3c, b3, jaeger
    #[serde(default = "default_propagation")]
    pub propagation: String,

    /// Custom span attributes using CEL expressions
    #[serde(default)]
    pub span_attributes: Vec<SpanAttribute>,
}

fn default_true() -> bool {
    true
}

fn default_service_name() -> String {
    "agentaskit-gateway".to_string()
}

fn default_sampling_rate() -> f64 {
    1.0
}

fn default_propagation() -> String {
    "w3c".to_string()
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            otlp_endpoint: None,
            service_name: default_service_name(),
            sampling_rate: default_sampling_rate(),
            propagation: default_propagation(),
            span_attributes: Vec::new(),
        }
    }
}

/// Custom span attribute using CEL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanAttribute {
    /// Attribute name
    pub name: String,

    /// CEL expression to compute the value
    pub expression: String,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable Prometheus metrics
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Metrics endpoint path
    #[serde(default = "default_metrics_path")]
    pub path: String,

    /// Include histogram buckets
    #[serde(default)]
    pub histogram_buckets: Vec<f64>,

    /// Custom labels using CEL
    #[serde(default)]
    pub custom_labels: Vec<MetricLabel>,

    /// OTLP endpoint for metric export
    #[serde(default)]
    pub otlp_endpoint: Option<String>,
}

fn default_metrics_path() -> String {
    "/metrics".to_string()
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            path: default_metrics_path(),
            histogram_buckets: vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0],
            custom_labels: Vec::new(),
            otlp_endpoint: None,
        }
    }
}

/// Custom metric label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricLabel {
    /// Label name
    pub name: String,

    /// CEL expression to compute the value
    pub expression: String,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Enable structured logging
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Log level: trace, debug, info, warn, error
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Log format: json, text
    #[serde(default = "default_log_format")]
    pub format: String,

    /// Include request/response bodies in logs
    #[serde(default)]
    pub include_bodies: bool,

    /// Custom log fields using CEL
    #[serde(default)]
    pub custom_fields: Vec<LogField>,

    /// Fields to redact from logs
    #[serde(default)]
    pub redact_fields: Vec<String>,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "json".to_string()
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: default_log_level(),
            format: default_log_format(),
            include_bodies: false,
            custom_fields: Vec::new(),
            redact_fields: vec![
                "authorization".to_string(),
                "x-api-key".to_string(),
                "password".to_string(),
            ],
        }
    }
}

/// Custom log field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogField {
    /// Field name
    pub name: String,

    /// CEL expression to compute the value
    pub expression: String,
}

/// Pre-defined CEL expressions for observability
pub mod cel_expressions {
    /// User agent header
    pub const USER_AGENT: &str = "request.headers[\"user-agent\"]";

    /// Client IP from X-Forwarded-For or connection
    pub const CLIENT_IP: &str = "request.headers[\"x-forwarded-for\"] ?? connection.source_ip";

    /// Request ID
    pub const REQUEST_ID: &str = "request.headers[\"x-request-id\"] ?? uuid()";

    /// JWT subject (user ID)
    pub const USER_ID: &str = "jwt.sub";

    /// MCP tool name
    pub const MCP_TOOL: &str = "mcp.tool.name";

    /// MCP resource URI
    pub const MCP_RESOURCE: &str = "mcp.resource.uri";

    /// A2A task ID
    pub const A2A_TASK_ID: &str = "a2a.task.id";

    /// Response status code
    pub const STATUS_CODE: &str = "response.code";

    /// Response latency in ms
    pub const LATENCY_MS: &str = "response.duration_ms";
}

/// Builder for observability configuration
#[derive(Debug, Clone)]
pub struct ObservabilityBuilder {
    config: ObservabilityConfig,
}

impl ObservabilityBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: ObservabilityConfig::default(),
        }
    }

    /// Set OTLP endpoint for all telemetry
    pub fn otlp_endpoint(mut self, endpoint: &str) -> Self {
        self.config.tracing.otlp_endpoint = Some(endpoint.to_string());
        self.config.metrics.otlp_endpoint = Some(endpoint.to_string());
        self
    }

    /// Set service name
    pub fn service_name(mut self, name: &str) -> Self {
        self.config.tracing.service_name = name.to_string();
        self
    }

    /// Set sampling rate
    pub fn sampling_rate(mut self, rate: f64) -> Self {
        self.config.tracing.sampling_rate = rate.clamp(0.0, 1.0);
        self
    }

    /// Set log level
    pub fn log_level(mut self, level: &str) -> Self {
        self.config.logging.level = level.to_string();
        self
    }

    /// Add custom span attribute
    pub fn span_attribute(mut self, name: &str, expression: &str) -> Self {
        self.config.tracing.span_attributes.push(SpanAttribute {
            name: name.to_string(),
            expression: expression.to_string(),
        });
        self
    }

    /// Add custom metric label
    pub fn metric_label(mut self, name: &str, expression: &str) -> Self {
        self.config.metrics.custom_labels.push(MetricLabel {
            name: name.to_string(),
            expression: expression.to_string(),
        });
        self
    }

    /// Add custom log field
    pub fn log_field(mut self, name: &str, expression: &str) -> Self {
        self.config.logging.custom_fields.push(LogField {
            name: name.to_string(),
            expression: expression.to_string(),
        });
        self
    }

    /// Add service attribute
    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.config.custom_attributes.insert(key.to_string(), value.to_string());
        self
    }

    /// Disable tracing
    pub fn disable_tracing(mut self) -> Self {
        self.config.tracing.enabled = false;
        self
    }

    /// Disable metrics
    pub fn disable_metrics(mut self) -> Self {
        self.config.metrics.enabled = false;
        self
    }

    /// Build the configuration
    pub fn build(self) -> ObservabilityConfig {
        self.config
    }
}

impl Default for ObservabilityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observability_builder() {
        let config = ObservabilityBuilder::new()
            .service_name("my-service")
            .otlp_endpoint("http://localhost:4317")
            .sampling_rate(0.5)
            .log_level("debug")
            .span_attribute("user_id", cel_expressions::USER_ID)
            .build();

        assert_eq!(config.tracing.service_name, "my-service");
        assert_eq!(config.tracing.sampling_rate, 0.5);
        assert_eq!(config.logging.level, "debug");
        assert_eq!(config.tracing.span_attributes.len(), 1);
    }

    #[test]
    fn test_default_configs() {
        let tracing = TracingConfig::default();
        assert!(tracing.enabled);
        assert_eq!(tracing.sampling_rate, 1.0);

        let metrics = MetricsConfig::default();
        assert!(metrics.enabled);
        assert_eq!(metrics.path, "/metrics");

        let logging = LoggingConfig::default();
        assert!(logging.enabled);
        assert_eq!(logging.level, "info");
    }
}
