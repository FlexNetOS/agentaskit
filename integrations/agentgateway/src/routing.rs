//! Routing configuration for agent-to-gateway communication
//!
//! This module provides routing configuration that maps AgentasKit
//! agents to agentgateway targets and listeners.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified routing configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// Route definitions
    #[serde(default)]
    pub routes: Vec<Route>,

    /// Default route (fallback)
    #[serde(default)]
    pub default_route: Option<String>,

    /// Virtual hosts for multiplexing
    #[serde(default)]
    pub virtual_hosts: Vec<VirtualHost>,
}

/// Route definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    /// Route name
    pub name: String,

    /// Match conditions
    pub matches: Vec<RouteMatch>,

    /// Target backend
    pub target: String,

    /// Route filters/transformations
    #[serde(default)]
    pub filters: Vec<RouteFilter>,

    /// Route timeout
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Retry configuration
    #[serde(default)]
    pub retry: Option<RetryConfig>,
}

/// Route match condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RouteMatch {
    /// Match by path prefix
    #[serde(rename = "path_prefix")]
    PathPrefix { prefix: String },

    /// Match by exact path
    #[serde(rename = "path_exact")]
    PathExact { path: String },

    /// Match by path regex
    #[serde(rename = "path_regex")]
    PathRegex { pattern: String },

    /// Match by header
    #[serde(rename = "header")]
    Header {
        name: String,
        #[serde(default)]
        value: Option<String>,
        #[serde(default)]
        pattern: Option<String>,
    },

    /// Match by MCP tool name
    #[serde(rename = "mcp_tool")]
    McpTool { name: String },

    /// Match by MCP resource URI
    #[serde(rename = "mcp_resource")]
    McpResource { uri_pattern: String },

    /// Match by A2A skill
    #[serde(rename = "a2a_skill")]
    A2aSkill { skill_id: String },

    /// Custom CEL expression match
    #[serde(rename = "cel")]
    Cel { expression: String },
}

/// Route filter/transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RouteFilter {
    /// Add request header
    #[serde(rename = "add_header")]
    AddHeader { name: String, value: String },

    /// Remove request header
    #[serde(rename = "remove_header")]
    RemoveHeader { name: String },

    /// Set request header (add or replace)
    #[serde(rename = "set_header")]
    SetHeader { name: String, value: String },

    /// URL rewrite
    #[serde(rename = "url_rewrite")]
    UrlRewrite {
        #[serde(default)]
        path_prefix: Option<String>,
        #[serde(default)]
        host: Option<String>,
    },

    /// Rate limit filter
    #[serde(rename = "rate_limit")]
    RateLimit {
        requests_per_second: u32,
        #[serde(default)]
        burst: Option<u32>,
    },

    /// Request timeout
    #[serde(rename = "timeout")]
    Timeout { ms: u64 },

    /// Custom CEL transformation
    #[serde(rename = "cel_transform")]
    CelTransform { expression: String },
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Number of retries
    pub attempts: u32,

    /// Per-attempt timeout in ms
    #[serde(default)]
    pub per_attempt_timeout_ms: Option<u64>,

    /// Retry on these status codes
    #[serde(default)]
    pub retry_on: Vec<u16>,

    /// Backoff configuration
    #[serde(default)]
    pub backoff: Option<BackoffConfig>,
}

/// Backoff configuration for retries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackoffConfig {
    /// Base delay in ms
    pub base_ms: u64,

    /// Maximum delay in ms
    #[serde(default)]
    pub max_ms: Option<u64>,

    /// Jitter factor (0.0 - 1.0)
    #[serde(default)]
    pub jitter: f64,
}

/// Virtual host for multiplexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualHost {
    /// Virtual host name
    pub name: String,

    /// Domains to match
    pub domains: Vec<String>,

    /// Routes for this virtual host
    pub routes: Vec<String>,
}

/// Route builder for fluent configuration
#[derive(Debug, Clone)]
pub struct RouteBuilder {
    route: Route,
}

impl RouteBuilder {
    /// Create a new route builder
    pub fn new(name: &str, target: &str) -> Self {
        Self {
            route: Route {
                name: name.to_string(),
                matches: Vec::new(),
                target: target.to_string(),
                filters: Vec::new(),
                timeout_ms: None,
                retry: None,
            },
        }
    }

    /// Add path prefix match
    pub fn path_prefix(mut self, prefix: &str) -> Self {
        self.route.matches.push(RouteMatch::PathPrefix {
            prefix: prefix.to_string(),
        });
        self
    }

    /// Add exact path match
    pub fn path_exact(mut self, path: &str) -> Self {
        self.route.matches.push(RouteMatch::PathExact {
            path: path.to_string(),
        });
        self
    }

    /// Add header match
    pub fn header(mut self, name: &str, value: Option<&str>) -> Self {
        self.route.matches.push(RouteMatch::Header {
            name: name.to_string(),
            value: value.map(|v| v.to_string()),
            pattern: None,
        });
        self
    }

    /// Add MCP tool match
    pub fn mcp_tool(mut self, tool_name: &str) -> Self {
        self.route.matches.push(RouteMatch::McpTool {
            name: tool_name.to_string(),
        });
        self
    }

    /// Add MCP resource match
    pub fn mcp_resource(mut self, uri_pattern: &str) -> Self {
        self.route.matches.push(RouteMatch::McpResource {
            uri_pattern: uri_pattern.to_string(),
        });
        self
    }

    /// Add A2A skill match
    pub fn a2a_skill(mut self, skill_id: &str) -> Self {
        self.route.matches.push(RouteMatch::A2aSkill {
            skill_id: skill_id.to_string(),
        });
        self
    }

    /// Add CEL expression match
    pub fn cel_match(mut self, expression: &str) -> Self {
        self.route.matches.push(RouteMatch::Cel {
            expression: expression.to_string(),
        });
        self
    }

    /// Add header filter
    pub fn add_header(mut self, name: &str, value: &str) -> Self {
        self.route.filters.push(RouteFilter::AddHeader {
            name: name.to_string(),
            value: value.to_string(),
        });
        self
    }

    /// Set timeout
    pub fn timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    /// Add rate limit
    pub fn rate_limit(mut self, rps: u32, burst: Option<u32>) -> Self {
        self.route.filters.push(RouteFilter::RateLimit {
            requests_per_second: rps,
            burst,
        });
        self
    }

    /// Configure retries
    pub fn retry(mut self, attempts: u32) -> Self {
        self.route.retry = Some(RetryConfig {
            attempts,
            per_attempt_timeout_ms: None,
            retry_on: vec![500, 502, 503, 504],
            backoff: Some(BackoffConfig {
                base_ms: 100,
                max_ms: Some(10000),
                jitter: 0.1,
            }),
        });
        self
    }

    /// Build the route
    pub fn build(self) -> Route {
        self.route
    }
}

/// Map AgentasKit agents to gateway routes
#[derive(Debug, Clone, Default)]
pub struct AgentRouteMapper {
    routes: HashMap<String, Route>,
}

impl AgentRouteMapper {
    /// Create a new route mapper
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Register an MCP agent
    pub fn register_mcp_agent(&mut self, agent_name: &str, target: &str) {
        let route = RouteBuilder::new(&format!("mcp-{}", agent_name), target)
            .path_prefix(&format!("/mcp/{}", agent_name))
            .build();
        self.routes.insert(agent_name.to_string(), route);
    }

    /// Register an A2A agent
    pub fn register_a2a_agent(&mut self, agent_name: &str, target: &str) {
        let route = RouteBuilder::new(&format!("a2a-{}", agent_name), target)
            .path_prefix(&format!("/a2a/{}", agent_name))
            .build();
        self.routes.insert(agent_name.to_string(), route);
    }

    /// Get all routes
    pub fn routes(&self) -> Vec<Route> {
        self.routes.values().cloned().collect()
    }

    /// Generate routing config
    pub fn to_config(&self) -> RoutingConfig {
        RoutingConfig {
            routes: self.routes(),
            default_route: None,
            virtual_hosts: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_builder() {
        let route = RouteBuilder::new("test-route", "test-target")
            .path_prefix("/api")
            .mcp_tool("calculator")
            .add_header("X-Custom", "value")
            .timeout(5000)
            .retry(3)
            .build();

        assert_eq!(route.name, "test-route");
        assert_eq!(route.target, "test-target");
        assert_eq!(route.matches.len(), 2);
        assert_eq!(route.filters.len(), 1);
        assert!(route.retry.is_some());
    }

    #[test]
    fn test_agent_route_mapper() {
        let mut mapper = AgentRouteMapper::new();
        mapper.register_mcp_agent("calculator", "calc-server");
        mapper.register_a2a_agent("assistant", "assistant-server");

        let config = mapper.to_config();
        assert_eq!(config.routes.len(), 2);
    }

    #[test]
    fn test_route_match_serialization() {
        let match_cond = RouteMatch::McpTool {
            name: "add".to_string(),
        };
        let json = serde_json::to_string(&match_cond).unwrap();
        assert!(json.contains("mcp_tool"));
        assert!(json.contains("add"));
    }
}
