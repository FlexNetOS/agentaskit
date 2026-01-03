//! Authentication and authorization integration
//!
//! This module provides JWT/OAuth2 authentication and CEL-based
//! authorization policy integration.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// JWT configuration
    #[serde(default)]
    pub jwt: Option<JwtConfig>,

    /// OAuth2 configuration
    #[serde(default)]
    pub oauth2: Option<OAuth2Config>,

    /// Authorization policies
    #[serde(default)]
    pub policies: Vec<AuthorizationPolicy>,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Issuer URL
    pub issuer: String,

    /// JWKS URL for public key discovery
    #[serde(default)]
    pub jwks_url: Option<String>,

    /// Audience to validate
    #[serde(default)]
    pub audience: Option<String>,

    /// Required claims
    #[serde(default)]
    pub required_claims: Vec<String>,

    /// Clock skew tolerance in seconds
    #[serde(default = "default_clock_skew")]
    pub clock_skew_seconds: u64,
}

fn default_clock_skew() -> u64 {
    60
}

/// OAuth2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    /// Authorization server URL
    pub auth_server_url: String,

    /// Token endpoint
    #[serde(default)]
    pub token_endpoint: Option<String>,

    /// Authorization endpoint
    #[serde(default)]
    pub auth_endpoint: Option<String>,

    /// JWKS endpoint
    #[serde(default)]
    pub jwks_endpoint: Option<String>,

    /// Supported scopes
    #[serde(default)]
    pub scopes: Vec<String>,

    /// Client credentials (for service-to-service)
    #[serde(default)]
    pub client_credentials: Option<ClientCredentials>,
}

/// Client credentials for OAuth2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    #[serde(default)]
    pub client_secret: Option<String>,
}

/// Authorization policy using CEL expressions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationPolicy {
    /// Policy name
    pub name: String,

    /// Description
    #[serde(default)]
    pub description: Option<String>,

    /// Target selector (what this policy applies to)
    #[serde(default)]
    pub target: PolicyTarget,

    /// CEL expression for the policy rule
    pub rule: String,

    /// Action when rule matches
    #[serde(default)]
    pub action: PolicyAction,
}

/// Policy target selector
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyTarget {
    /// Target listeners
    #[serde(default)]
    pub listeners: Vec<String>,

    /// Target routes
    #[serde(default)]
    pub routes: Vec<String>,

    /// Target methods (for MCP: tools, resources, prompts)
    #[serde(default)]
    pub methods: Vec<String>,
}

/// Policy action
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PolicyAction {
    #[default]
    Allow,
    Deny,
    Audit,
}

/// CEL expression builder for authorization policies
#[derive(Debug, Clone)]
pub struct CelPolicyBuilder {
    expressions: Vec<String>,
}

impl CelPolicyBuilder {
    /// Create a new CEL policy builder
    pub fn new() -> Self {
        Self {
            expressions: Vec::new(),
        }
    }

    /// Require a specific JWT claim
    pub fn require_claim(mut self, claim: &str, value: &str) -> Self {
        self.expressions.push(format!("jwt.{} == \"{}\"", claim, value));
        self
    }

    /// Require JWT subject
    pub fn require_subject(self, subject: &str) -> Self {
        self.require_claim("sub", subject)
    }

    /// Require specific scope
    pub fn require_scope(mut self, scope: &str) -> Self {
        self.expressions.push(format!("\"{}\" in jwt.scope", scope));
        self
    }

    /// Require any of the specified scopes
    pub fn require_any_scope(mut self, scopes: &[&str]) -> Self {
        let scope_checks: Vec<String> = scopes.iter()
            .map(|s| format!("\"{}\" in jwt.scope", s))
            .collect();
        self.expressions.push(format!("({})", scope_checks.join(" || ")));
        self
    }

    /// Require all of the specified scopes
    pub fn require_all_scopes(mut self, scopes: &[&str]) -> Self {
        let scope_checks: Vec<String> = scopes.iter()
            .map(|s| format!("\"{}\" in jwt.scope", s))
            .collect();
        self.expressions.push(format!("({})", scope_checks.join(" && ")));
        self
    }

    /// Restrict to specific MCP tool
    pub fn mcp_tool(mut self, tool_name: &str) -> Self {
        self.expressions.push(format!("mcp.tool.name == \"{}\"", tool_name));
        self
    }

    /// Restrict to specific MCP resource
    pub fn mcp_resource(mut self, resource_uri: &str) -> Self {
        self.expressions.push(format!("mcp.resource.uri == \"{}\"", resource_uri));
        self
    }

    /// Add custom CEL expression
    pub fn custom(mut self, expression: &str) -> Self {
        self.expressions.push(expression.to_string());
        self
    }

    /// Build the final CEL expression
    pub fn build(self) -> String {
        if self.expressions.is_empty() {
            "true".to_string()
        } else {
            self.expressions.join(" && ")
        }
    }
}

impl Default for CelPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Common CEL expressions for MCP authorization
pub mod cel_expressions {
    /// Allow all requests (permissive)
    pub const ALLOW_ALL: &str = "true";

    /// Deny all requests (restrictive)
    pub const DENY_ALL: &str = "false";

    /// Require authenticated user
    pub const REQUIRE_AUTH: &str = "has(jwt.sub)";

    /// Require specific issuer
    pub fn require_issuer(issuer: &str) -> String {
        format!("jwt.iss == \"{}\"", issuer)
    }

    /// Require user from specific audience
    pub fn require_audience(audience: &str) -> String {
        format!("\"{}\" in jwt.aud", audience)
    }

    /// Rate limit check (requests per second)
    pub fn rate_limit(requests_per_second: u32) -> String {
        format!("ratelimit.requests_per_second <= {}", requests_per_second)
    }

    /// Time-based access (business hours)
    pub const BUSINESS_HOURS: &str = "now.getHours() >= 9 && now.getHours() < 17";

    /// MCP tool access by name pattern
    pub fn mcp_tool_pattern(pattern: &str) -> String {
        format!("mcp.tool.name.matches(\"{}\")", pattern)
    }

    /// MCP resource access by URI pattern
    pub fn mcp_resource_pattern(pattern: &str) -> String {
        format!("mcp.resource.uri.matches(\"{}\")", pattern)
    }
}

/// Token validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidation {
    pub valid: bool,
    pub subject: Option<String>,
    pub issuer: Option<String>,
    pub audience: Vec<String>,
    pub scopes: Vec<String>,
    pub claims: HashMap<String, serde_json::Value>,
    pub expires_at: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cel_policy_builder() {
        let policy = CelPolicyBuilder::new()
            .require_subject("user-123")
            .require_scope("read:tools")
            .mcp_tool("calculator")
            .build();

        assert!(policy.contains("jwt.sub == \"user-123\""));
        assert!(policy.contains("\"read:tools\" in jwt.scope"));
        assert!(policy.contains("mcp.tool.name == \"calculator\""));
    }

    #[test]
    fn test_cel_policy_empty() {
        let policy = CelPolicyBuilder::new().build();
        assert_eq!(policy, "true");
    }

    #[test]
    fn test_policy_action_default() {
        let action: PolicyAction = Default::default();
        assert!(matches!(action, PolicyAction::Allow));
    }

    #[test]
    fn test_cel_expressions() {
        assert_eq!(cel_expressions::ALLOW_ALL, "true");
        assert_eq!(cel_expressions::DENY_ALL, "false");

        let issuer_check = cel_expressions::require_issuer("https://auth.example.com");
        assert!(issuer_check.contains("jwt.iss"));
    }
}
