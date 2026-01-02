# AgentasKit Gateway Integration

This module provides integration between AgentasKit and [Agentgateway](https://github.com/agentgateway/agentgateway), enabling MCP (Model Context Protocol), A2A (Agent2Agent), and other protocol support for agent communication.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        AgentasKit                                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Agent 1    │  │   Agent 2    │  │   Agent N    │          │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘          │
│         │                 │                 │                   │
│  ┌──────┴─────────────────┴─────────────────┴──────┐           │
│  │            Gateway Integration Layer            │           │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌───────┐ │           │
│  │  │   MCP   │ │   A2A   │ │  Auth   │ │Routing│ │           │
│  │  └─────────┘ └─────────┘ └─────────┘ └───────┘ │           │
│  └──────────────────────┬──────────────────────────┘           │
└─────────────────────────┼───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Agentgateway                                │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │  Listeners  │  │   Targets   │  │  Policies   │             │
│  │  (MCP/A2A)  │  │(stdio/HTTP) │  │(auth/rate)  │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
└─────────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    External Services                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │ MCP Servers │  │ A2A Agents  │  │ HTTP APIs   │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
└─────────────────────────────────────────────────────────────────┘
```

## Features

- **MCP Support**: Connect to MCP servers (stdio, SSE, HTTP) for tool execution
- **A2A Protocol**: Inter-agent communication with task management
- **CEL Authorization**: Fine-grained access control using Common Expression Language
- **XDS Configuration**: Dynamic configuration updates from control plane
- **OpenTelemetry**: Distributed tracing, metrics, and logging
- **Rate Limiting**: Global, per-user, and per-target rate limits
- **JWT/OAuth2**: Token-based authentication and authorization

## Quick Start

### 1. Add Dependency

```toml
[dependencies]
agentaskit-gateway-integration = { path = "integrations/agentgateway" }
```

### 2. Start the Gateway

```rust
use agentaskit_gateway_integration::{
    config::{GatewayConfig, TargetConfig},
    gateway::GatewayManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let mut config = GatewayConfig::default();

    // Add an MCP server target
    config.targets.push(TargetConfig {
        name: "my-mcp-server".to_string(),
        target_type: "stdio".to_string(),
        command: Some("npx".to_string()),
        args: vec!["-y".to_string(), "@modelcontextprotocol/server-everything".to_string()],
        host: None,
    });

    // Create and start gateway manager
    let mut manager = GatewayManager::new(config);
    manager.start().await?;

    println!("Gateway UI: {}", manager.admin_url());

    // Keep running...
    tokio::signal::ctrl_c().await?;
    manager.stop().await?;

    Ok(())
}
```

### 3. Use MCP Client

```rust
use agentaskit_gateway_integration::mcp::McpClient;

async fn call_mcp_tool() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("localhost:8080", "my-mcp-server");

    // List available tools
    let tools = client.list_tools().await?;
    println!("Available tools: {:?}", tools);

    // Call a tool
    let result = client.call_tool("add", serde_json::json!({
        "a": 1,
        "b": 2
    })).await?;
    println!("Result: {:?}", result);

    Ok(())
}
```

### 4. Use A2A Client

```rust
use agentaskit_gateway_integration::a2a::{A2aClient, TaskRequest};

async fn send_a2a_task() -> Result<(), Box<dyn std::error::Error>> {
    let client = A2aClient::new("localhost:8081", "my-agent");

    // Get agent capabilities
    let card = client.get_agent_card().await?;
    println!("Agent: {} - Skills: {:?}", card.name, card.skills);

    // Send a task
    let task = TaskRequest::text("Hello, can you help me?")
        .with_session("session-123");

    let response = client.send_task(&task).await?;
    println!("Task status: {:?}", response.status.state);

    Ok(())
}
```

## Configuration

### Local Development

Use `configs/agentgateway/local.yaml`:

```yaml
version: v1

listeners:
  - name: mcp-listener
    protocol: mcp
    address: "127.0.0.1:8080"

targets:
  - name: my-server
    type: stdio
    command: npx
    args: ["-y", "@modelcontextprotocol/server-everything"]
```

### Production

Use `configs/agentgateway/production.yaml` with environment variables:

```yaml
mcpAuthentication:
  issuer: "${OAUTH_ISSUER_URL}"
  jwksUrl: "${OAUTH_JWKS_URL}"
```

## Authorization with CEL

```rust
use agentaskit_gateway_integration::auth::{AuthorizationPolicy, CelPolicyBuilder};

// Build a policy
let policy = CelPolicyBuilder::new()
    .require_subject("user-123")
    .require_scope("read:tools")
    .mcp_tool("calculator")
    .build();

// Results in: jwt.sub == "user-123" && "read:tools" in jwt.scope && mcp.tool.name == "calculator"
```

## Rate Limiting

```rust
use agentaskit_gateway_integration::ratelimit::{RateLimitBuilder, policies};

let config = RateLimitBuilder::new()
    .global(100, Some(150))           // 100 req/s global
    .per_user(10, Some(20))           // 10 req/s per user
    .target("expensive-api", 5, None) // 5 req/s for specific target
    .policy(policies::mcp_tool_limit())
    .build();
```

## Observability

```rust
use agentaskit_gateway_integration::observability::ObservabilityBuilder;

let config = ObservabilityBuilder::new()
    .service_name("my-agent-service")
    .otlp_endpoint("http://localhost:4317")
    .sampling_rate(0.5)
    .span_attribute("user_id", "jwt.sub")
    .build();
```

## Modules

| Module | Description |
|--------|-------------|
| `config` | Gateway configuration types |
| `gateway` | Process manager for agentgateway |
| `mcp` | MCP client and server configuration |
| `a2a` | A2A protocol client and types |
| `auth` | JWT/OAuth2 and CEL authorization |
| `routing` | Route configuration and mapping |
| `ratelimit` | Rate limiting policies |
| `observability` | OpenTelemetry integration |
| `xds` | XDS dynamic configuration |

## Building Agentgateway

The agentgateway binary must be built from the submodule:

```bash
cd agentgateway
npm install && npm run build  # Build UI
make build                     # Build binary
```

The binary will be at `agentgateway/target/release/agentgateway`.

## License

MIT OR Apache-2.0
