# AgentAsKit Documentation

## Structure

```
docs/
├── architecture/          # System architecture documentation
│   └── agent_hierarchical_map.md
├── dev/                   # Developer documentation
│   ├── BUILD.md          # Build system documentation
│   └── AGENT_GATEWAY_INTEGRATION.md  # Gateway integration guide
├── guides/               # User guides and tutorials
│   ├── agent.chatmode.md
│   ├── mode1-noa-dynamic-ui-cross-platform.md
│   └── tools.md
├── decisions/            # Architecture Decision Records (ADRs)
├── observability/        # Logging and monitoring docs
├── ops/                  # Operations documentation
├── perf/                 # Performance documentation
├── reports/              # Generated reports
├── runbooks/             # Operational runbooks
└── security/             # Security documentation
```

## Key Documents

| Document | Description |
|----------|-------------|
| [BUILD.md](dev/BUILD.md) | Build system and compilation guide |
| [AGENT_GATEWAY_INTEGRATION.md](dev/AGENT_GATEWAY_INTEGRATION.md) | Agent gateway architecture |
| [agent_hierarchical_map.md](architecture/agent_hierarchical_map.md) | Six-layer agent hierarchy |

## Integrations

- **aichat**: Unified AI CLI - see `integrations/aichat/`
- **claude-flow**: Orchestration layer - see `integrations/claude-flow/`
- **kellnr**: Local crate registry - see `integrations/kellnr/`
- **agentgateway**: MCP/A2A protocol gateway - see `integrations/agentgateway/`
