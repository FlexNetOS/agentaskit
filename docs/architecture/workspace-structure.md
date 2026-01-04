# AgentAsKit Workspace Structure

## Overview

AgentAsKit uses a **dual-workspace architecture** that separates development from production. This document explains the reasoning, structure, and usage of both workspaces.

## The Dual-Workspace Model

### Root Workspace (`/agentaskit/`)

**Purpose:** Active development environment where new features and agents are built.

**Characteristics:**
- Contains 297 Rust source files (more than production)
- Includes experimental cargo-specific agents
- Development tools and build infrastructure
- Git submodule references to integrations
- Pixi/mise for cross-platform development

**Key Components:**
```
├── core/                    - Main application (2.5M)
│   └── 297 Rust files including new cargo agents
├── shared/                  - Shared types library (78K)
├── unified_agents/          - Agent factory directory (8.1M)
├── unified_tools/           - Build/verification tools (45K)
├── unified_orchestration/   - Orchestration policies (21K)
├── configs/                 - Configuration files
├── operational_*/           - Runtime systems
└── [Integrations via git submodules]
```

**Tools:**
- Pixi: Environment and package management
- Nushell: Primary shell for cross-platform scripting
- Cargo: Rust build system
- mise: Optional task runner
- GitHub Actions: CI/CD pipelines

**Development Workflow:**
```
Developer → Root Workspace → git commit → CI/CD
    ↓
  Code review
    ↓
Merge to main → Build Production Package
```

---

### Production Workspace (`/agentaskit-production/`)

**Purpose:** Release-ready deployment package with verified and tested code.

**Characteristics:**
- Contains 138 Rust source files (stable subset)
- Frozen snapshot of verified code
- Complete deployment infrastructure
- Kubernetes/Helm configurations
- WASM execution environment

**Key Components:**
```
├── core/                    - Stable subset (138 Rust files)
├── shared/                  - Identical to root
├── services/                - Health monitoring (UNIQUE to production)
│   └── Health check APIs
├── unified_execution/       - WASM execution host (UNIQUE)
│   ├── core/               - Execution engine
│   └── wasm_host/          - WASM runtime
├── deploy/                  - Deployment manifests (UNIQUE)
│   ├── k8s/                - Kubernetes configs
│   ├── helm/               - Helm charts
│   └── canary/             - Canary deployment plans
└── .github/                 - CI/CD pipeline
```

**Deployment Targets:**
- Kubernetes clusters (K8s manifests)
- Helm package deployments
- Container orchestration (OCI)
- Cloud providers (AWS, GCP, Azure)

**Release Workflow:**
```
Production Package → Build → Test → Container Image
    ↓
Push to Registry → Deploy to K8s → Health Check
    ↓
Monitor → Alerts → Rollback (if needed)
```

---

## Key Differences Summary

| Aspect | Root Workspace | Production Package |
|--------|----------------|-------------------|
| **Purpose** | Active development | Release deployment |
| **Rust Files** | 297 (includes experimental) | 138 (stable subset) |
| **Includes** | unified_orchestration | unified_execution (WASM) |
| **Services** | None | Health monitoring services |
| **Deployment** | Local/CI testing | Kubernetes/Helm |
| **Build Type** | Debug + Release | Release only |
| **Git Submodules** | All initialized | Only deployment tools |
| **Development Tools** | Pixi, mise, direnv | Container registry |

---

## Workspace Dependencies and Flow

### Build Pipeline
```
┌─────────────────────────────────────┐
│     ROOT WORKSPACE (Development)     │
│                                      │
│  ┌──────────────────────────────┐   │
│  │ Cargo Workspace Members       │   │
│  │ - core/ (297 files)          │   │
│  │ - shared/ (types)             │   │
│  │ - unified_tools/              │   │
│  │ - unified_agents/             │   │
│  │ - unified_orchestration/      │   │
│  └──────────────────────────────┘   │
│                │                      │
│                ↓                      │
│  ┌──────────────────────────────┐   │
│  │ Testing & Validation         │   │
│  │ - cargo test                  │   │
│  │ - cargo clippy                │   │
│  │ - config validation           │   │
│  └──────────────────────────────┘   │
│                │                      │
│                ↓                      │
│  ┌──────────────────────────────┐   │
│  │ Build Artifacts              │   │
│  │ - Release binaries            │   │
│  │ - Config snapshots            │   │
│  │ - SBOM generation            │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
           │
           ├─→ CI/CD Validation
           │
           ↓
┌─────────────────────────────────────┐
│ PRODUCTION PACKAGE (Release)         │
│                                      │
│  ┌──────────────────────────────┐   │
│  │ Deployment Components         │   │
│  │ - core/ (138 files, stable)  │   │
│  │ - services/ (NEW)             │   │
│  │ - unified_execution/ (NEW)   │   │
│  │ - deploy/ (K8s, Helm, etc)   │   │
│  └──────────────────────────────┘   │
│                │                      │
│                ↓                      │
│  ┌──────────────────────────────┐   │
│  │ Container Image Build         │   │
│  │ - Docker image construction   │   │
│  │ - Vulnerability scanning      │   │
│  │ - Artifact signing            │   │
│  └──────────────────────────────┘   │
│                │                      │
│                ↓                      │
│  ┌──────────────────────────────┐   │
│  │ Kubernetes Deployment         │   │
│  │ - K8s manifests               │   │
│  │ - Helm charts                 │   │
│  │ - Configuration injection     │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
```

---

## File Organization

### Configuration Files

**Root-level (Development):**
```
Cargo.toml                - Workspace root (2 members: core, shared)
Cargo.lock               - Development dependencies
pixi.toml               - Environment manager (Python, Rust, Nushell, Node)
.mise.toml              - Task runner
.envrc                  - direnv configuration
rustfmt.toml            - Rust formatting rules
.aider.conf.yml         - Aider AI pair programming config
Makefile                - Build automation
```

**configs/ (Shared):**
```
configs/
├── nushell/             - Shell environment (env.nu, pixi-activate.nu)
├── agentgateway/        - MCP/A2A protocol configs
├── wiki/                - Wiki-rs configurations
├── production/          - Resource specifications
├── tools/               - Config management tools
│   ├── validate_config.py
│   ├── generate_config.py
│   └── migrate_config.py
├── rate_limits.yaml     - Rate limiting rules
└── tracing.yaml         - OpenTelemetry configuration
```

### Production-Only

**agentaskit-production/**
```
Dockerfile              - Container image definition
Cargo.toml             - Workspace root (5 members)
deploy/
├── k8s/                - Kubernetes manifests
├── helm/               - Helm charts
└── canary/             - Canary deployment strategies
docker-compose.yml     - Local composition
.github/
├── workflows/          - Complete CI/CD pipeline
└── actions/            - Custom GitHub actions
```

---

## Cargo Workspace Structure

### Root Workspace (Cargo.toml)

```toml
[workspace]
members = ["core", "shared"]
resolver = "2"

[workspace.package]
version = "1.0.0"
edition = "2021"
authors = ["AgentAsKit Team"]
```

**Members:**
- `core`: Main binary application
- `shared`: Shared type library

### Production Workspace (agentaskit-production/Cargo.toml)

```toml
[workspace]
members = [
    "core",
    "shared",
    "services",
    "unified_execution/core",
    "unified_execution/wasm_host"
]
```

**Additional Members:**
- `services`: Health monitoring and observability
- `unified_execution/core`: WASM execution engine
- `unified_execution/wasm_host`: WASM runtime host

---

## Integrations (Git Submodules)

All integrations are git submodules that need initialization:

```bash
git submodule update --init --recursive
```

### Core Integrations

| Name | Purpose | Status |
|------|---------|--------|
| `agentgateway` | MCP/A2A protocol routing | Critical |
| `wiki-rs` | Documentation engine | Critical |
| `integrations/aichat` | CLI for 20+ AI providers | Active |
| `integrations/claude-flow` | Multi-agent orchestration | Active |
| `integrations/kellnr` | Private Rust crate registry | Active |
| `integrations/llama.cpp` | Local LLM inference | Active |

---

## Development vs Production Deployment

### Development Deployment

**Target:** Local machines, CI/CD runners, staging environments

**Process:**
```bash
# 1. Development environment setup
pixi shell
. pixi-activate.nu

# 2. Build and test
cargo build
cargo test
cargo clippy

# 3. Run locally
cargo run -- --help
```

**Configuration:** Uses dev configs from `configs/agentgateway/agentgateway-dev.yaml`

### Production Deployment

**Target:** Kubernetes clusters, managed container services

**Process:**
```bash
# 1. Build container image
docker build -t agentaskit:1.0.0 .

# 2. Push to registry
docker push registry.example.com/agentaskit:1.0.0

# 3. Deploy via Helm
helm install agentaskit ./deploy/helm/agentaskit \
  -f deploy/helm/values-production.yaml

# 4. Verify health
kubectl get pods -l app=agentaskit
kubectl logs -f deployment/agentaskit
```

**Configuration:** Uses production configs from `configs/agentgateway/agentgateway-prod.yaml`

---

## Operational Systems

### Audit Trail
```
operational_audit/
├── README.md                    - Audit documentation
├── unification_audit.md         - System audit reports
└── [Generated audit logs]       - Runtime audit records
```

### Integrity Verification
```
operational_hash/
├── README.md                    - Hash documentation
├── generate_integrity.py        - Manifest generator
└── system_integrity.json        - Hash verification records
```

### System Management
```
operational_scripts/
├── backup.sh                    - System backup automation
├── monitor.sh                   - Health monitoring
├── deploy.sh                    - Deployment automation
└── system_manager.sh            - System operations
```

---

## Cross-Platform Support via Pixi + Nushell

### Pixi Environment Management

**Supports multiple languages:**
- Python 3.11+
- Rust 1.70+
- Nushell 0.90+
- Node.js 20+ + pnpm 8+

**Platform support:**
- Linux (x86_64)
- macOS (ARM64, x86_64)
- Windows (x86_64)

### Nushell as Primary Shell

**Why Nushell?**
- Cross-platform compatibility (replaces Bash)
- Structured data pipelines
- Type-safe scripting
- Better Windows support

**Example scripts:**
```
tools/bootstrap.nu          - Project initialization
tools/lint.nu               - Linting automation
tools/verify.nu             - Verification pipeline
```

---

## Configuration Management

### Configuration Tools

**Validation:**
```bash
python3 configs/tools/validate_config.py configs/tracing.yaml
```

**Generation:**
```bash
python3 configs/tools/generate_config.py all -e production
```

**Migration:**
```bash
python3 configs/tools/migrate_config.py migrate configs/old.yaml -v 2.0.0
```

### Environment-Specific Configs

```
configs/
├── base/               - Shared defaults
├── dev/                - Development overrides
├── staging/            - Staging overrides
└── production/         - Production overrides
```

---

## Feature Flags and Build Modes

### Available Features

```toml
[features]
default = []

# Inference engines
inference-llama = []
inference-candle = ["candle-core", "candle-nn"]
inference-burn = ["burn"]
ml-vector = ["fastembed", "qdrant-client"]

# Deployment modes
desktop = ["tauri"]
server = []
embedded = []

# Integrations
mcp-protocol = []
wasm-execution = []
```

### Building with Features

```bash
# Enable local LLM inference
cargo build --features inference-llama

# Enable vector search
cargo build --features ml-vector

# Complete ML stack
cargo build --features inference-llama,inference-candle,ml-vector

# Production deployment
cargo build --release --features wasm-execution,mcp-protocol
```

---

## Migration and Versioning

### Workspace Versioning

Both workspaces use the same version (synchronized in Cargo.toml workspace config):
- Development: `1.0.0` (root)
- Production: `1.0.0` (agentaskit-production)

### Dependency Management

- **Root Workspace**: Uses `Cargo.lock` for reproducible builds
- **Production Package**: Has separate `Cargo.lock` for deployment consistency

### Updating Dependencies

```bash
# In root workspace
cargo update
cargo tree

# Verify no breaking changes
cargo test --all

# If all good, update production
cp Cargo.lock agentaskit-production/
```

---

## Best Practices

### When to Work in Root Workspace

✅ **Do work in root when:**
- Adding new features or agents
- Improving cargo-specific tooling
- Experimenting with new integrations
- Running tests and CI validation
- Developing new agents

### When Production Package is Updated

✅ **Do update production when:**
- Root passes all tests
- Code is ready for release
- Configuration is stable
- Performance is verified
- Security scan passes

### Submodule Management

✅ **Do initialize submodules:**
```bash
git submodule update --init --recursive
```

❌ **Don't:**
- Manually edit submodule contents
- Commit changes to submodule pointers without intent
- Forget to push submodule changes separately

---

## Troubleshooting

### Issue: Submodule not initialized

**Solution:**
```bash
git submodule update --init --recursive
```

### Issue: Cargo.lock conflicts

**Solution:**
```bash
# Keep both lock files in sync
cargo update
cp Cargo.lock agentaskit-production/
git add Cargo.lock agentaskit-production/Cargo.lock
```

### Issue: Config validation failures

**Solution:**
```bash
# Check config against schema
python3 configs/tools/validate_config.py configs/your-config.yaml

# Generate valid config from template
python3 configs/tools/generate_config.py your-config -e dev -o configs/dev/
```

---

## Related Documentation

- [Feature Flags](../features.md) - Detailed feature flag documentation
- [LLaMA.cpp Integration](./llama-cpp.md) - Local inference setup
- [Deployment Guide](../../deploy/README.md) - Kubernetes deployment
- [Configuration Guide](../../configs/README.md) - Configuration management
- [CI/CD Pipeline](.github/workflows/README.md) - GitHub Actions workflow

---

## Summary

AgentAsKit's dual-workspace architecture provides:

✅ **Development Flexibility** - Root workspace for active development
✅ **Production Stability** - Frozen, tested deployments
✅ **Clear Separation** - Obvious distinction between dev and prod
✅ **Scalability** - Supports multiple deployment targets
✅ **Cross-Platform** - Pixi + Nushell for Windows/Mac/Linux
✅ **Type Safety** - Cargo workspace member isolation
✅ **Infrastructure** - Complete K8s/Helm deployment support

This structure ensures that development work doesn't interfere with production stability, while maintaining a clear path from development to release.
