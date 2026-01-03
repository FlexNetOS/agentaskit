# Agent Gateway Integration Architecture

## Overview

This document describes how the AgentAsKit agent gateway integrates with the orchestration layer (claude-flow) and the centralized AI CLI (aichat).

## Current Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              AgentAsKit                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                        configs/                                      │    │
│  │   ├── nushell/env.nu          # Cross-platform environment          │    │
│  │   ├── agentgateway/local.yaml # Gateway configuration               │    │
│  │   └── aider/                  # Aider configuration                 │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                       integrations/                                  │    │
│  │   ├── agentgateway/           # MCP/A2A protocol gateway            │    │
│  │   │   ├── mcp.rs              # MCP client                          │    │
│  │   │   ├── a2a.rs              # Agent-to-Agent protocol             │    │
│  │   │   ├── routing.rs          # Route matching & mapping            │    │
│  │   │   ├── auth.rs             # CEL-based authorization             │    │
│  │   │   └── config.rs           # Gateway configuration               │    │
│  │   │                                                                  │    │
│  │   ├── claude-flow/            # Orchestration layer (TypeScript)    │    │
│  │   │   ├── src/mcp/            # MCP server implementations          │    │
│  │   │   ├── src/agents/         # Agent definitions                   │    │
│  │   │   ├── src/coordination/   # Multi-agent coordination            │    │
│  │   │   ├── src/memory/         # Memory/context management           │    │
│  │   │   └── src/hive-mind/      # Swarm intelligence                  │    │
│  │   │                                                                  │    │
│  │   ├── aichat/                 # Centralized AI CLI (Rust)           │    │
│  │   │   ├── src/client/         # Provider implementations            │    │
│  │   │   │   ├── openai.rs       # OpenAI                              │    │
│  │   │   │   ├── claude.rs       # Anthropic Claude                    │    │
│  │   │   │   ├── gemini.rs       # Google Gemini                       │    │
│  │   │   │   ├── bedrock.rs      # AWS Bedrock                         │    │
│  │   │   │   ├── vertexai.rs     # Google Vertex AI                    │    │
│  │   │   │   └── cohere.rs       # Cohere                              │    │
│  │   │   ├── src/config/         # Configuration                       │    │
│  │   │   ├── src/repl/           # Interactive REPL                    │    │
│  │   │   └── scripts/completions/aichat.nu  # Nushell integration      │    │
│  │   │                                                                  │    │
│  │   └── llama.cpp/              # Local inference                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                          core/                                       │    │
│  │   ├── src/orchestration/      # Rust orchestration layer            │    │
│  │   │   ├── orchestrator.rs     # Autonomous orchestrator             │    │
│  │   │   ├── agent.rs            # Agent definitions                   │    │
│  │   │   ├── broker.rs           # Message broker                      │    │
│  │   │   ├── engine.rs           # Execution engine                    │    │
│  │   │   └── hootl.rs            # Human-on-the-loop                   │    │
│  │   ├── src/execution/          # Task execution                      │    │
│  │   ├── src/ai/                 # AI bridges                          │    │
│  │   │   ├── sop_analyzer.rs     # SOP analysis                        │    │
│  │   │   └── model_selector_bridge.rs  # llama.cpp bridge              │    │
│  │   ├── src/ui/                 # UI (Tauri/Web/API)                  │    │
│  │   └── src/config/             # Configuration system                │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Integration Points

### 1. Provider Connections (via aichat)

The aichat CLI provides a unified interface to all AI providers:

**Supported Providers:**
- OpenAI (GPT-4, GPT-3.5)
- Anthropic (Claude)
- Google (Gemini, Vertex AI)
- AWS (Bedrock)
- Cohere
- 18+ OpenAI-compatible providers (Groq, Mistral, DeepSeek, etc.)

**Integration:**
```rust
// In core/src/ai/provider_bridge.rs (TO BE CREATED)
pub struct ProviderBridge {
    aichat_path: PathBuf,
}

impl ProviderBridge {
    pub async fn complete(&self, prompt: &str, provider: &str) -> Result<String> {
        // Call aichat CLI
        Command::new(&self.aichat_path)
            .args(["-m", provider, prompt])
            .output()
            .await
    }
}
```

### 2. Tool Connections (via MCP)

Tools connect through the agentgateway MCP layer:

**Current:**
- Static tool definitions in `configs/agentgateway/local.yaml`
- `McpClient` in `integrations/agentgateway/src/mcp.rs`

**Planned Enhancement:**
- Dynamic MCP server spawning per tool
- Tool registry with metadata
- CEL-based tool authorization

### 3. NOA/AI Connection

**Currently Missing** - Needs implementation:

```
noa/
├── ai/           # AI orchestration
│   ├── mod.rs
│   ├── provider.rs    # Provider abstraction
│   └── inference.rs   # Inference management
├── config/       # NOA configuration
│   ├── mod.rs
│   └── schema.rs      # Config schema
└── shared/       # Shared utilities
    ├── mod.rs
    └── types.rs       # Common types
```

### 4. Orchestration Layer Integration

claude-flow provides TypeScript orchestration capabilities:

**Integration via MCP:**
```yaml
# configs/agentgateway/orchestration.yaml
targets:
  - name: claude-flow-orchestrator
    type: stdio
    command: node
    args: ["integrations/claude-flow/bin/claude-flow.js", "mcp"]

listeners:
  - name: orchestration-listener
    protocol: mcp
    address: "127.0.0.1:8082"
```

**Or via native Rust integration:**
```rust
// core/src/orchestration/claude_flow_bridge.rs (TO BE CREATED)
pub struct ClaudeFlowBridge {
    process: Child,
    websocket: WebSocket,
}
```

### 5. Sandboxing (MISSING)

Needs implementation:
- Use landlock (Linux) or sandbox-exec (macOS) for process isolation
- Container-based sandboxing for tools
- Resource limits via cgroups

### 6. Local Inference

Currently via llama.cpp:
- Feature-gated in `core/src/ai/model_selector_bridge.rs`
- Uses `LLAMA_CPP_PATH` environment variable

**Enhancement:** Integrate with aichat's local inference support.

### 7. UI Bridge

Current structure in `core/src/ui/`:
- `DesktopApp` - Tauri desktop (placeholder)
- `WebServer` - Axum web (placeholder)
- `ApiServer` - Axum API (placeholder)

### 8. Configuration Integration

The new nushell config (`configs/nushell/env.nu`) sets environment variables that are picked up by `ConfigManager::load_from_env()`.

## Recommended Integration Steps

### Phase 1: aichat Integration
1. Add aichat as workspace member in Cargo.toml
2. Create `core/src/ai/provider_bridge.rs`
3. Wire aichat to agentgateway as MCP target

### Phase 2: claude-flow Integration
1. Configure claude-flow as MCP server in agentgateway
2. Create WebSocket bridge for real-time coordination
3. Integrate hive-mind for multi-agent orchestration

### Phase 3: NOA Layer
1. Create `noa/` directory structure
2. Implement config schema
3. Bridge to both aichat and claude-flow

### Phase 4: Sandboxing
1. Implement landlock-based sandboxing for Linux
2. Add container support for tools
3. Implement resource limits

## Configuration Example

```yaml
# configs/agentgateway/production.yaml
listeners:
  - name: mcp-main
    protocol: mcp
    address: "127.0.0.1:8080"
  - name: a2a-main
    protocol: a2a
    address: "127.0.0.1:8081"
  - name: orchestration
    protocol: mcp
    address: "127.0.0.1:8082"

targets:
  # aichat as AI provider
  - name: aichat-provider
    type: stdio
    command: aichat
    args: ["--serve", "--port", "8083"]

  # claude-flow orchestrator
  - name: claude-flow
    type: stdio
    command: node
    args: ["integrations/claude-flow/bin/claude-flow.js", "mcp"]

  # Local inference
  - name: llama-local
    type: stdio
    command: ./integrations/llama.cpp/main
    args: ["-m", "models/model.gguf"]

routing:
  routes:
    - name: ai-requests
      matches:
        - type: mcp_tool
          name: "ai_*"
      target: aichat-provider

    - name: orchestration
      matches:
        - type: path_prefix
          prefix: "/orchestrate"
      target: claude-flow

    - name: local-inference
      matches:
        - type: header
          name: "X-Provider"
          value: "local"
      target: llama-local

auth:
  policies:
    - name: allow-tools
      condition: "request.path.startsWith('/mcp/')"
      action: allow
```

## Nushell Integration

```nu
# configs/nushell/env.nu additions

# aichat integration
def ai [prompt: string, --model: string = "claude"] {
    ^aichat -m $model $prompt
}

# claude-flow orchestration
def orchestrate [task: string] {
    ^node integrations/claude-flow/bin/claude-flow.js run $task
}

# Provider selection
def "ai providers" [] {
    ^aichat --list-models | lines | parse "{provider}:{model}"
}
```

## Next Steps

1. [ ] Create `core/src/ai/provider_bridge.rs` for aichat integration
2. [ ] Add aichat binary to `tools/bin/`
3. [ ] Configure claude-flow as MCP target
4. [ ] Create `noa/` module structure
5. [ ] Implement sandboxing layer
6. [ ] Add production gateway configuration
7. [ ] Create integration tests
