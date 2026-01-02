//! Authentication and authorization tests

#[path = "../src/auth.rs"]
mod auth;

use auth::*;

#[test]
fn test_cel_policy_builder_basic() {
    let policy = CelPolicyBuilder::new()
        .require_subject("user-123")
        .build();

    assert!(policy.contains("jwt.sub"));
    assert!(policy.contains("user-123"));
}

#[test]
fn test_cel_policy_builder_scope() {
    let policy = CelPolicyBuilder::new()
        .require_scope("read:tools")
        .build();

    assert!(policy.contains("read:tools"));
    assert!(policy.contains("jwt.scope"));
}

#[test]
fn test_cel_policy_builder_any_scope() {
    let policy = CelPolicyBuilder::new()
        .require_any_scope(&["read:tools", "write:tools"])
        .build();

    assert!(policy.contains("read:tools"));
    assert!(policy.contains("write:tools"));
    assert!(policy.contains("||"));
}

#[test]
fn test_cel_policy_builder_all_scopes() {
    let policy = CelPolicyBuilder::new()
        .require_all_scopes(&["read:tools", "write:tools"])
        .build();

    assert!(policy.contains("read:tools"));
    assert!(policy.contains("write:tools"));
    assert!(policy.contains("&&"));
}

#[test]
fn test_cel_policy_builder_mcp_tool() {
    let policy = CelPolicyBuilder::new()
        .mcp_tool("calculator")
        .build();

    assert!(policy.contains("mcp.tool.name"));
    assert!(policy.contains("calculator"));
}

#[test]
fn test_cel_policy_builder_mcp_resource() {
    let policy = CelPolicyBuilder::new()
        .mcp_resource("file:///project/src")
        .build();

    assert!(policy.contains("mcp.resource.uri"));
}

#[test]
fn test_cel_policy_builder_combined() {
    let policy = CelPolicyBuilder::new()
        .require_subject("user-123")
        .require_scope("admin")
        .mcp_tool("dangerous-tool")
        .custom("request.method == \"POST\"")
        .build();

    assert!(policy.contains("jwt.sub"));
    assert!(policy.contains("admin"));
    assert!(policy.contains("mcp.tool.name"));
    assert!(policy.contains("request.method"));
    assert!(policy.contains("&&"));
}

#[test]
fn test_cel_policy_builder_empty() {
    let policy = CelPolicyBuilder::new().build();
    assert_eq!(policy, "true");
}

#[test]
fn test_authorization_policy() {
    let policy = AuthorizationPolicy {
        name: "admin-only".to_string(),
        description: Some("Only allow admin users".to_string()),
        target: PolicyTarget {
            listeners: vec!["admin-listener".to_string()],
            routes: vec![],
            methods: vec![],
        },
        rule: "jwt.role == \"admin\"".to_string(),
        action: PolicyAction::Allow,
    };

    assert_eq!(policy.name, "admin-only");
    assert!(matches!(policy.action, PolicyAction::Allow));
}

#[test]
fn test_policy_action_default() {
    let action: PolicyAction = Default::default();
    assert!(matches!(action, PolicyAction::Allow));
}

#[test]
fn test_jwt_config() {
    let config = JwtConfig {
        issuer: "https://auth.example.com".to_string(),
        jwks_url: Some("https://auth.example.com/.well-known/jwks.json".to_string()),
        audience: Some("my-api".to_string()),
        required_claims: vec!["sub".to_string(), "email".to_string()],
        clock_skew_seconds: 60,
    };

    assert!(config.jwks_url.is_some());
    assert_eq!(config.required_claims.len(), 2);
    assert_eq!(config.clock_skew_seconds, 60);
}

#[test]
fn test_oauth2_config() {
    let config = OAuth2Config {
        auth_server_url: "https://auth.example.com".to_string(),
        token_endpoint: Some("/oauth/token".to_string()),
        auth_endpoint: Some("/oauth/authorize".to_string()),
        jwks_endpoint: Some("/.well-known/jwks.json".to_string()),
        scopes: vec!["openid".to_string(), "profile".to_string()],
        client_credentials: Some(ClientCredentials {
            client_id: "my-client".to_string(),
            client_secret: Some("secret".to_string()),
        }),
    };

    assert!(config.client_credentials.is_some());
    assert_eq!(config.scopes.len(), 2);
}

#[test]
fn test_policy_target() {
    let target = PolicyTarget {
        listeners: vec!["mcp-listener".to_string()],
        routes: vec!["api/*".to_string()],
        methods: vec!["tools/call".to_string()],
    };

    assert_eq!(target.listeners.len(), 1);
    assert_eq!(target.routes.len(), 1);
    assert_eq!(target.methods.len(), 1);
}

#[test]
fn test_cel_expressions_constants() {
    assert_eq!(cel_expressions::ALLOW_ALL, "true");
    assert_eq!(cel_expressions::DENY_ALL, "false");
    assert_eq!(cel_expressions::REQUIRE_AUTH, "has(jwt.sub)");
}

#[test]
fn test_cel_expressions_functions() {
    let issuer = cel_expressions::require_issuer("https://auth.example.com");
    assert!(issuer.contains("jwt.iss"));
    assert!(issuer.contains("https://auth.example.com"));

    let audience = cel_expressions::require_audience("my-api");
    assert!(audience.contains("jwt.aud"));
    assert!(audience.contains("my-api"));

    let rate_limit = cel_expressions::rate_limit(100);
    assert!(rate_limit.contains("100"));

    let tool_pattern = cel_expressions::mcp_tool_pattern("calc.*");
    assert!(tool_pattern.contains("matches"));

    let resource_pattern = cel_expressions::mcp_resource_pattern("file://.*");
    assert!(resource_pattern.contains("mcp.resource.uri"));
}

#[test]
fn test_token_validation() {
    let validation = TokenValidation {
        valid: true,
        subject: Some("user-123".to_string()),
        issuer: Some("https://auth.example.com".to_string()),
        audience: vec!["my-api".to_string()],
        scopes: vec!["read".to_string(), "write".to_string()],
        claims: std::collections::HashMap::new(),
        expires_at: Some(1704067200),
    };

    assert!(validation.valid);
    assert!(validation.subject.is_some());
    assert_eq!(validation.scopes.len(), 2);
}

#[test]
fn test_auth_config() {
    let config = AuthConfig {
        jwt: Some(JwtConfig {
            issuer: "https://auth.example.com".to_string(),
            jwks_url: None,
            audience: None,
            required_claims: vec![],
            clock_skew_seconds: 60,
        }),
        oauth2: None,
        policies: vec![
            AuthorizationPolicy {
                name: "default".to_string(),
                description: None,
                target: PolicyTarget::default(),
                rule: "true".to_string(),
                action: PolicyAction::Allow,
            }
        ],
    };

    assert!(config.jwt.is_some());
    assert!(config.oauth2.is_none());
    assert_eq!(config.policies.len(), 1);
}
