//! Rate limiting configuration for agent communication
//!
//! This module provides rate limiting policies that can be applied
//! to agent-to-agent and agent-to-tool communication through agentgateway.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rate limiting configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Global rate limits
    #[serde(default)]
    pub global: Option<GlobalRateLimit>,

    /// Per-target rate limits
    #[serde(default)]
    pub targets: HashMap<String, TargetRateLimit>,

    /// Per-user rate limits
    #[serde(default)]
    pub per_user: Option<PerUserRateLimit>,

    /// Rate limit policies
    #[serde(default)]
    pub policies: Vec<RateLimitPolicy>,
}

/// Global rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalRateLimit {
    /// Requests per second
    pub requests_per_second: u32,

    /// Burst size
    #[serde(default)]
    pub burst: Option<u32>,

    /// Response when rate limited
    #[serde(default)]
    pub response: RateLimitResponse,
}

/// Per-target rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetRateLimit {
    /// Target name pattern
    #[serde(default)]
    pub name_pattern: Option<String>,

    /// Requests per second
    pub requests_per_second: u32,

    /// Burst size
    #[serde(default)]
    pub burst: Option<u32>,

    /// Queue configuration
    #[serde(default)]
    pub queue: Option<QueueConfig>,
}

/// Per-user rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerUserRateLimit {
    /// Requests per second per user
    pub requests_per_second: u32,

    /// Burst size per user
    #[serde(default)]
    pub burst: Option<u32>,

    /// Key extraction CEL expression
    #[serde(default = "default_user_key")]
    pub key_expression: String,
}

fn default_user_key() -> String {
    "jwt.sub".to_string()
}

/// Rate limit policy with CEL selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitPolicy {
    /// Policy name
    pub name: String,

    /// CEL selector for when to apply
    pub selector: String,

    /// Rate limit configuration
    pub limit: PolicyLimit,

    /// Key extraction for per-key limiting
    #[serde(default)]
    pub key: Option<String>,

    /// Priority (lower = higher priority)
    #[serde(default)]
    pub priority: u32,
}

/// Rate limit for a policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyLimit {
    /// Requests allowed
    pub requests: u32,

    /// Time window
    pub window: TimeWindow,

    /// Burst allowance
    #[serde(default)]
    pub burst: Option<u32>,
}

/// Time window for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "unit", content = "value")]
pub enum TimeWindow {
    #[serde(rename = "second")]
    Second(u32),
    #[serde(rename = "minute")]
    Minute(u32),
    #[serde(rename = "hour")]
    Hour(u32),
    #[serde(rename = "day")]
    Day(u32),
}

impl TimeWindow {
    /// Convert to seconds
    pub fn to_seconds(&self) -> u64 {
        match self {
            TimeWindow::Second(n) => *n as u64,
            TimeWindow::Minute(n) => *n as u64 * 60,
            TimeWindow::Hour(n) => *n as u64 * 3600,
            TimeWindow::Day(n) => *n as u64 * 86400,
        }
    }
}

/// Queue configuration for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    /// Maximum queue size
    pub max_size: u32,

    /// Queue timeout in ms
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Prioritization
    #[serde(default)]
    pub priority: QueuePriority,
}

/// Queue priority mode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueuePriority {
    /// First-in, first-out
    #[default]
    Fifo,
    /// Last-in, first-out
    Lifo,
    /// Priority-based (requires priority header)
    Priority,
}

/// Response when rate limited
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResponse {
    /// HTTP status code
    #[serde(default = "default_status_code")]
    pub status_code: u16,

    /// Response body
    #[serde(default)]
    pub body: Option<String>,

    /// Include retry-after header
    #[serde(default = "default_true")]
    pub include_retry_after: bool,

    /// Custom headers
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

fn default_status_code() -> u16 {
    429
}

fn default_true() -> bool {
    true
}

impl Default for RateLimitResponse {
    fn default() -> Self {
        Self {
            status_code: 429,
            body: Some(r#"{"error": "rate_limited", "message": "Too many requests"}"#.to_string()),
            include_retry_after: true,
            headers: HashMap::new(),
        }
    }
}

/// Builder for rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitBuilder {
    config: RateLimitConfig,
}

impl RateLimitBuilder {
    /// Create a new rate limit builder
    pub fn new() -> Self {
        Self {
            config: RateLimitConfig::default(),
        }
    }

    /// Set global rate limit
    pub fn global(mut self, rps: u32, burst: Option<u32>) -> Self {
        self.config.global = Some(GlobalRateLimit {
            requests_per_second: rps,
            burst,
            response: RateLimitResponse::default(),
        });
        self
    }

    /// Add per-target rate limit
    pub fn target(mut self, target: &str, rps: u32, burst: Option<u32>) -> Self {
        self.config.targets.insert(
            target.to_string(),
            TargetRateLimit {
                name_pattern: None,
                requests_per_second: rps,
                burst,
                queue: None,
            },
        );
        self
    }

    /// Set per-user rate limit
    pub fn per_user(mut self, rps: u32, burst: Option<u32>) -> Self {
        self.config.per_user = Some(PerUserRateLimit {
            requests_per_second: rps,
            burst,
            key_expression: default_user_key(),
        });
        self
    }

    /// Add a rate limit policy
    pub fn policy(mut self, policy: RateLimitPolicy) -> Self {
        self.config.policies.push(policy);
        self
    }

    /// Build the configuration
    pub fn build(self) -> RateLimitConfig {
        self.config
    }
}

impl Default for RateLimitBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Pre-defined rate limit policies
pub mod policies {
    use super::*;

    /// High-volume API policy (100 req/s)
    pub fn high_volume() -> RateLimitPolicy {
        RateLimitPolicy {
            name: "high-volume".to_string(),
            selector: "true".to_string(),
            limit: PolicyLimit {
                requests: 100,
                window: TimeWindow::Second(1),
                burst: Some(150),
            },
            key: None,
            priority: 100,
        }
    }

    /// Standard API policy (10 req/s per user)
    pub fn standard_per_user() -> RateLimitPolicy {
        RateLimitPolicy {
            name: "standard-per-user".to_string(),
            selector: "has(jwt.sub)".to_string(),
            limit: PolicyLimit {
                requests: 10,
                window: TimeWindow::Second(1),
                burst: Some(20),
            },
            key: Some("jwt.sub".to_string()),
            priority: 50,
        }
    }

    /// MCP tool rate limit (5 req/s per tool)
    pub fn mcp_tool_limit() -> RateLimitPolicy {
        RateLimitPolicy {
            name: "mcp-tool-limit".to_string(),
            selector: "has(mcp.tool.name)".to_string(),
            limit: PolicyLimit {
                requests: 5,
                window: TimeWindow::Second(1),
                burst: Some(10),
            },
            key: Some("mcp.tool.name".to_string()),
            priority: 25,
        }
    }

    /// A2A task rate limit (1 req/s per session)
    pub fn a2a_session_limit() -> RateLimitPolicy {
        RateLimitPolicy {
            name: "a2a-session-limit".to_string(),
            selector: "has(a2a.session_id)".to_string(),
            limit: PolicyLimit {
                requests: 60,
                window: TimeWindow::Minute(1),
                burst: Some(10),
            },
            key: Some("a2a.session_id".to_string()),
            priority: 25,
        }
    }

    /// Expensive operation policy (slow rate)
    pub fn expensive_operation(selector: &str) -> RateLimitPolicy {
        RateLimitPolicy {
            name: "expensive-operation".to_string(),
            selector: selector.to_string(),
            limit: PolicyLimit {
                requests: 10,
                window: TimeWindow::Minute(1),
                burst: Some(2),
            },
            key: Some("jwt.sub".to_string()),
            priority: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_time_window() {
        assert_eq!(TimeWindow::Second(1).to_seconds(), 1);
        assert_eq!(TimeWindow::Minute(1).to_seconds(), 60);
        assert_eq!(TimeWindow::Hour(1).to_seconds(), 3600);
        assert_eq!(TimeWindow::Day(1).to_seconds(), 86400);
    }

    #[test]
    fn test_predefined_policies() {
        let policy = policies::high_volume();
        assert_eq!(policy.name, "high-volume");

        let user_policy = policies::standard_per_user();
        assert!(user_policy.key.is_some());
    }
}
