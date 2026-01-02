//! XDS dynamic configuration support
//!
//! This module provides integration with the XDS (xDS Transport Protocol)
//! for dynamic configuration updates from a control plane.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// XDS configuration for connecting to a control plane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsConfig {
    /// Control plane address
    pub control_plane_address: String,

    /// Node ID for this gateway instance
    #[serde(default = "default_node_id")]
    pub node_id: String,

    /// Cluster name
    #[serde(default = "default_cluster")]
    pub cluster: String,

    /// Resource types to subscribe to
    #[serde(default)]
    pub resource_types: Vec<XdsResourceType>,

    /// TLS configuration for control plane connection
    #[serde(default)]
    pub tls: Option<XdsTlsConfig>,

    /// Retry configuration
    #[serde(default)]
    pub retry: XdsRetryConfig,
}

fn default_node_id() -> String {
    format!("agentaskit-gateway-{}", uuid::Uuid::new_v4())
}

fn default_cluster() -> String {
    "agentaskit-cluster".to_string()
}

impl Default for XdsConfig {
    fn default() -> Self {
        Self {
            control_plane_address: "localhost:18000".to_string(),
            node_id: default_node_id(),
            cluster: default_cluster(),
            resource_types: vec![
                XdsResourceType::Listener,
                XdsResourceType::Route,
                XdsResourceType::Target,
                XdsResourceType::Policy,
            ],
            tls: None,
            retry: XdsRetryConfig::default(),
        }
    }
}

/// XDS resource types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum XdsResourceType {
    /// Listener resources
    Listener,
    /// Route resources
    Route,
    /// Target/backend resources
    Target,
    /// Policy resources (auth, ratelimit, etc.)
    Policy,
    /// Workload resources
    Workload,
}

/// TLS configuration for XDS connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsTlsConfig {
    /// CA certificate for verifying control plane
    pub ca_cert: String,

    /// Client certificate for mTLS
    #[serde(default)]
    pub client_cert: Option<String>,

    /// Client key for mTLS
    #[serde(default)]
    pub client_key: Option<String>,
}

/// Retry configuration for XDS connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsRetryConfig {
    /// Initial retry delay in ms
    #[serde(default = "default_initial_delay")]
    pub initial_delay_ms: u64,

    /// Maximum retry delay in ms
    #[serde(default = "default_max_delay")]
    pub max_delay_ms: u64,

    /// Maximum number of retries (0 = infinite)
    #[serde(default)]
    pub max_retries: u32,
}

fn default_initial_delay() -> u64 {
    1000
}

fn default_max_delay() -> u64 {
    30000
}

impl Default for XdsRetryConfig {
    fn default() -> Self {
        Self {
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            max_retries: 0,
        }
    }
}

/// XDS resource for listeners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsListener {
    /// Resource name
    pub name: String,

    /// Protocol type
    pub protocol: String,

    /// Bind address
    pub address: String,

    /// Associated policies
    #[serde(default)]
    pub policies: Vec<String>,

    /// Metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// XDS resource for routes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsRoute {
    /// Resource name
    pub name: String,

    /// Parent listener
    pub listener: String,

    /// Target backend
    pub target: String,

    /// Match conditions
    #[serde(default)]
    pub matches: Vec<XdsRouteMatch>,

    /// Route priority
    #[serde(default)]
    pub priority: u32,
}

/// XDS route match condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsRouteMatch {
    /// Match type
    #[serde(rename = "type")]
    pub match_type: String,

    /// Match value
    pub value: String,
}

/// XDS resource for targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsTarget {
    /// Resource name
    pub name: String,

    /// Target type
    #[serde(rename = "type")]
    pub target_type: String,

    /// Endpoints
    #[serde(default)]
    pub endpoints: Vec<XdsEndpoint>,

    /// Health check configuration
    #[serde(default)]
    pub health_check: Option<XdsHealthCheck>,
}

/// XDS endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsEndpoint {
    /// Address
    pub address: String,

    /// Port
    pub port: u16,

    /// Weight for load balancing
    #[serde(default = "default_weight")]
    pub weight: u32,

    /// Health status
    #[serde(default)]
    pub healthy: bool,
}

fn default_weight() -> u32 {
    1
}

/// XDS health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsHealthCheck {
    /// Interval between checks in seconds
    pub interval_seconds: u32,

    /// Timeout for each check in seconds
    pub timeout_seconds: u32,

    /// Number of consecutive failures before unhealthy
    pub unhealthy_threshold: u32,

    /// Number of consecutive successes before healthy
    pub healthy_threshold: u32,

    /// Health check path (for HTTP)
    #[serde(default)]
    pub path: Option<String>,
}

/// XDS policy resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsPolicy {
    /// Resource name
    pub name: String,

    /// Policy type
    #[serde(rename = "type")]
    pub policy_type: String,

    /// Target references
    #[serde(default)]
    pub target_refs: Vec<XdsPolicyTargetRef>,

    /// Policy configuration
    pub config: serde_json::Value,
}

/// XDS policy target reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XdsPolicyTargetRef {
    /// Reference kind (Listener, Route, Target)
    pub kind: String,

    /// Reference name
    pub name: String,

    /// Section name (optional)
    #[serde(default)]
    pub section_name: Option<String>,
}

/// Builder for XDS configuration
#[derive(Debug, Clone)]
pub struct XdsConfigBuilder {
    config: XdsConfig,
}

impl XdsConfigBuilder {
    /// Create a new builder
    pub fn new(control_plane_address: &str) -> Self {
        Self {
            config: XdsConfig {
                control_plane_address: control_plane_address.to_string(),
                ..Default::default()
            },
        }
    }

    /// Set node ID
    pub fn node_id(mut self, id: &str) -> Self {
        self.config.node_id = id.to_string();
        self
    }

    /// Set cluster name
    pub fn cluster(mut self, name: &str) -> Self {
        self.config.cluster = name.to_string();
        self
    }

    /// Add resource type subscription
    pub fn subscribe(mut self, resource_type: XdsResourceType) -> Self {
        if !self.config.resource_types.contains(&resource_type) {
            self.config.resource_types.push(resource_type);
        }
        self
    }

    /// Configure TLS
    pub fn tls(mut self, ca_cert: &str) -> Self {
        self.config.tls = Some(XdsTlsConfig {
            ca_cert: ca_cert.to_string(),
            client_cert: None,
            client_key: None,
        });
        self
    }

    /// Configure mTLS
    pub fn mtls(mut self, ca_cert: &str, client_cert: &str, client_key: &str) -> Self {
        self.config.tls = Some(XdsTlsConfig {
            ca_cert: ca_cert.to_string(),
            client_cert: Some(client_cert.to_string()),
            client_key: Some(client_key.to_string()),
        });
        self
    }

    /// Build the configuration
    pub fn build(self) -> XdsConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xds_config_builder() {
        let config = XdsConfigBuilder::new("localhost:18000")
            .node_id("test-node")
            .cluster("test-cluster")
            .subscribe(XdsResourceType::Listener)
            .subscribe(XdsResourceType::Route)
            .build();

        assert_eq!(config.control_plane_address, "localhost:18000");
        assert_eq!(config.node_id, "test-node");
        assert_eq!(config.cluster, "test-cluster");
        assert!(config.resource_types.contains(&XdsResourceType::Listener));
    }

    #[test]
    fn test_xds_config_default() {
        let config = XdsConfig::default();
        assert!(!config.node_id.is_empty());
        assert_eq!(config.cluster, "agentaskit-cluster");
    }

    #[test]
    fn test_xds_listener_serialization() {
        let listener = XdsListener {
            name: "test-listener".to_string(),
            protocol: "mcp".to_string(),
            address: "0.0.0.0:8080".to_string(),
            policies: vec!["auth-policy".to_string()],
            metadata: HashMap::new(),
        };

        let json = serde_json::to_string(&listener).unwrap();
        assert!(json.contains("test-listener"));
        assert!(json.contains("mcp"));
    }
}
