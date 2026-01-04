# AgentAsKit Cargo Features

This document provides comprehensive documentation for all available Cargo feature flags in AgentAsKit.

## Quick Reference

```toml
# Build with all features
cargo build --features "inference-llama,inference-candle,ml-vector,mcp-protocol,wasm-execution,desktop"

# Build for server deployment
cargo build --release --features "wasm-execution,mcp-protocol"

# Build with local inference
cargo build --features "inference-llama"

# Build for desktop application
cargo build --features "desktop"
```

## Feature Categories

### Inference Engine Features

Inference engines provide different machine learning capabilities for AI/LLM tasks.

#### `inference-llama`

**Status:** Recommended for production
**Dependencies:** `llama-cpp-rs`, libllama.cpp

**Purpose:** Enables local LLM inference via llama.cpp

**What it enables:**
- Local large language model inference
- GGUF model support
- CPU and GPU acceleration
- No external API calls required
- Model quantization support

**When to use:**
- Running models locally without cloud provider
- Privacy-sensitive deployments
- Cost optimization (no API charges)
- Offline operation capability
- Custom model fine-tuning

**Example usage:**
```rust
#[cfg(feature = "inference-llama")]
use agentaskit_core::ai::llama_bridge::LlamaInference;

let inference = LlamaInference::new("path/to/model.gguf")?;
let response = inference.complete("Your prompt here").await?;
```

**Build command:**
```bash
cargo build --features "inference-llama"
```

**Supported models:**
- Llama 2 (7B, 13B, 70B)
- Llama 3 (8B, 70B)
- Mistral (7B, 8x7B)
- Phi (2B, 3B, 14B)
- Other GGUF-compatible models

**GPU Support:**
- NVIDIA CUDA (automatic detection)
- Apple Metal (automatic on macOS)
- AMD ROCm (manual configuration)

---

#### `inference-candle`

**Status:** Experimental/Future use
**Dependencies:** `candle-core`, `candle-nn`

**Purpose:** Lightweight ML inference framework for production

**What it enables:**
- Fast inference with minimal dependencies
- WASM-compatible inference
- Quantized model support
- Memory-efficient computation
- WebAssembly support

**When to use:**
- Embedded systems
- WASM deployments
- Low-latency requirements
- Memory-constrained environments
- Browser-based inference

**Supported models:**
- Vision transformers
- Language models (smaller variants)
- Custom ONNX models
- Quantized neural networks

**Build command:**
```bash
cargo build --features "inference-candle"
```

**Note:** Currently declared but not actively used; for future inference engine expansion

---

#### `inference-burn`

**Status:** Experimental/Future use
**Dependencies:** `burn`

**Purpose:** Full-featured deep learning framework

**What it enables:**
- Complex neural network architectures
- Custom model training pipelines
- Advanced optimization algorithms
- Research-grade flexibility
- Multi-GPU training support

**When to use:**
- Custom model development
- Research and experimentation
- Complex training pipelines
- Advanced optimization needs
- Academic research

**Build command:**
```bash
cargo build --features "inference-burn"
```

**Note:** Declared for future use; can support custom model development

---

### ML and Vector Features

These features enable machine learning and vector search capabilities.

#### `ml-vector`

**Status:** Experimental/Future use
**Dependencies:** `fastembed`, `qdrant-client`

**Purpose:** Vector search and embeddings for semantic search and RAG

**What it enables:**
- Fast embedding generation
- Vector similarity search
- Retrieval-augmented generation (RAG)
- Semantic search capabilities
- Memory-efficient vector storage

**What it includes:**
- **fastembed:** Fast, optimized embedding models (BERT, E5, etc.)
- **qdrant-client:** Vector database client for similarity search

**When to use:**
- Knowledge base search
- Semantic document retrieval
- Similarity matching
- Context retrieval for LLMs (RAG)
- Recommendation systems

**Example usage:**
```rust
#[cfg(feature = "ml-vector")]
use fastembed::FlagEmbedding;
use qdrant_client::Qdrant;

let embeddings = FlagEmbedding::new(Default::default())?;
let docs_embed = embeddings.embed(vec!["Your document"], None)?;

let qdrant = Qdrant::from_url("http://localhost:6333").build()?;
// Search and retrieve similar documents
```

**Build command:**
```bash
cargo build --features "ml-vector"
```

**Supported embeddings:**
- BERT-based models
- E5 models (multilingual)
- FastBERT
- Custom ONNX embeddings

**Vector database:**
- Qdrant (open source)
- Cloud: Pinecone, Weaviate, Milvus

---

### Deployment Mode Features

These features specify the deployment context and available interfaces.

#### `desktop`

**Status:** Production ready
**Dependencies:** `tauri`, `tauri-build`

**Purpose:** Desktop application mode with Tauri UI framework

**What it enables:**
- Desktop application window
- Cross-platform native UI (Windows, macOS, Linux)
- System tray integration
- File system access
- Native system APIs

**When to use:**
- Building desktop applications
- ARK-OS Desktop application
- Cross-platform UI applications
- Local user applications

**App Details:**
- **Name:** ARK-OS Desktop
- **Version:** 0.1.0
- **Window Size:** 1200x800 (resizable)
- **Frontend:** Tauri-managed frontend

**Build command:**
```bash
cargo build --features "desktop"
```

**Desktop features included:**
- Window management
- System tray
- File operations
- Clipboard access
- Notifications
- Native dialogs

**Platform support:**
- Windows (7+)
- macOS (10.13+)
- Linux (GTK-based)

**Note:** This is the PRIMARY UI framework. If `desktop` is not used, what alternative UI framework is configured?

---

#### `server`

**Status:** Available
**Dependencies:** None (base functionality)

**Purpose:** Server/API mode without desktop UI

**What it enables:**
- Headless operation
- API-only deployment
- Microservice mode
- Container deployment (K8s)
- Background service

**When to use:**
- Running as a backend service
- Kubernetes deployments
- Docker containers
- Headless servers
- API integration points

**Build command:**
```bash
cargo build --release --features "server"
```

**Server interfaces:**
- gRPC API (via tonic)
- HTTP REST (via actix/axum if configured)
- MCP protocol (if mcp-protocol feature enabled)
- A2A protocol (via agentgateway)

---

#### `embedded`

**Status:** Available
**Dependencies:** None (library mode)

**Purpose:** Embedded/library mode for integration into other applications

**What it enables:**
- Library integration
- Foreign function interface (FFI)
- Embedded system deployment
- Plugin architecture support

**When to use:**
- Embedding in other Rust applications
- Building plugins
- Embedded system integration
- Shared library export

**Build command:**
```bash
cargo build --lib --features "embedded"
```

**Library exports:**
- Public API surface
- No binary entry point
- Integration-friendly modules

---

### Integration Features

#### `mcp-protocol`

**Status:** Recommended
**Dependencies:** `tonic`, `prost`

**Purpose:** Model Context Protocol support for agent communication

**What it enables:**
- MCP (Model Context Protocol) server
- Agent-to-agent communication
- Standardized protocol support
- Tool calling and context sharing
- Multi-agent coordination

**When to use:**
- Multi-agent systems
- Agent communication
- Tool integration
- Context sharing between agents
- Orchestration scenarios

**Protocols included:**
- **MCP:** Model Context Protocol (OpenAI standard)
- **A2A:** Agent-to-Agent (custom, via agentgateway)

**Build command:**
```bash
cargo build --features "mcp-protocol"
```

**Example usage:**
```rust
#[cfg(feature = "mcp-protocol")]
use agentaskit_core::ai::gateway_bridge::MCPBridge;

let mcp_server = MCPBridge::new()?;
// Register tools and handlers
```

---

#### `wasm-execution`

**Status:** Production ready
**Dependencies:** `wasmtime`, `wasi-common`

**Purpose:** WebAssembly runtime for sandboxed code execution

**What it enables:**
- WASM module execution
- Sandboxed computation
- Secure plugin execution
- Cross-platform bytecode
- Resource isolation

**When to use:**
- Executing untrusted code safely
- Plugin architecture
- Distributable computations
- Sandboxed user code
- Portable binary execution

**Build command:**
```bash
cargo build --features "wasm-execution"
```

**WASM capabilities:**
- WASI system interface
- Memory isolation
- CPU time limits
- Resource constraints
- Function imports/exports

**Example usage:**
```rust
#[cfg(feature = "wasm-execution")]
use agentaskit_core::execution::wasm_host::WasmHost;

let mut wasm = WasmHost::new("module.wasm")?;
let result = wasm.call("compute", args)?;
```

---

## Feature Combinations

### Recommended Feature Sets

#### Development Build
```bash
cargo build --features "inference-llama,ml-vector,desktop,mcp-protocol"
```

**Includes:** Local inference, vector search, desktop UI, agent communication

#### Server Deployment
```bash
cargo build --release --features "inference-llama,mcp-protocol,wasm-execution"
```

**Includes:** Local inference, agent protocols, sandboxed execution

#### Full ML Stack
```bash
cargo build --features "inference-llama,inference-candle,ml-vector"
```

**Includes:** Multiple inference engines, vector search, embeddings

#### Production Desktop
```bash
cargo build --release --features "desktop,inference-llama"
```

**Includes:** Desktop UI, local inference

---

## Feature Dependencies

### Dependency Graph

```
inference-candle ──────────┐
                           ├─→ ML Stack
inference-burn ────────────┤
                           │
ml-vector ────────────────┘

inference-llama (independent)
                     ↓
           [LLaMA.cpp Integration]

desktop ──────────────────────┐
server  ──────────────────────├─→ Deployment Mode
embedded ─────────────────────┘

mcp-protocol (independent)
wasm-execution (independent)
```

### Conflicting Features

⚠️ **Warning:** Some feature combinations may conflict:

- `desktop` + `server`: Choose one deployment mode
- Conflicting UI frameworks (if multiple configured): Only one should be active
- Resource constraints: `wasm-execution` + high memory inference may require tuning

---

## Enabling Features in Cargo.toml

### Project Dependencies

In your `Cargo.toml`:

```toml
[dependencies]
agentaskit-core = { path = ".", features = ["inference-llama", "mcp-protocol"] }
```

### Conditional Compilation

In your Rust code:

```rust
// Enable code only with inference-llama feature
#[cfg(feature = "inference-llama")]
mod llama_specific {
    // llama.cpp specific code
}

// Enable code with multiple features
#[cfg(all(feature = "inference-llama", feature = "ml-vector"))]
fn advanced_rag_pipeline() {
    // Combined inference + vector search
}
```

---

## Feature Flag States

### Active Features (Production Ready)
- ✅ `inference-llama` - Local LLM inference
- ✅ `desktop` - Desktop application mode
- ✅ `mcp-protocol` - Model Context Protocol
- ✅ `wasm-execution` - WebAssembly runtime

### Experimental Features (Under Development)
- 🟡 `inference-candle` - Lightweight ML framework
- 🟡 `ml-vector` - Vector search and embeddings

### Future Features (Planned)
- 📋 `inference-burn` - Deep learning framework
- 📋 `server` - Server/API mode (core support ready)
- 📋 `embedded` - Library embedding (core support ready)

---

## Building from Command Line

### Basic Builds

```bash
# Default build (no features)
cargo build

# With all features
cargo build --all-features

# With specific features
cargo build --features "inference-llama,desktop"

# Release build with features
cargo build --release --features "inference-llama,mcp-protocol,wasm-execution"
```

### Feature Testing

```bash
# Test with specific features
cargo test --features "inference-llama"

# Test all feature combinations
cargo test --all-features

# Bench with features
cargo bench --features "ml-vector"
```

### Feature Verification

```bash
# List active features
cargo metadata --format-version 1 | jq '.packages[0].features'

# Check feature tree
cargo tree --features "inference-llama,ml-vector"
```

---

## Troubleshooting

### Feature Not Recognized

**Problem:** Error about unknown feature

**Solution:**
1. Check feature spelling in Cargo.toml
2. Verify feature is defined in `[features]` section
3. Ensure you're in the correct crate directory

```bash
# Validate Cargo.toml
cargo metadata --format-version 1
```

### Missing Dependencies for Feature

**Problem:** Build fails when enabling a feature

**Solution:**
1. Feature dependencies are automatically resolved
2. Check internet connection for crate downloads
3. Clear cache and rebuild:

```bash
cargo clean
cargo build --features "your-feature"
```

### Conflicting Features

**Problem:** Two features are incompatible

**Solution:**
1. Check feature combinations in documentation
2. Use conditional dependencies if needed
3. Consider separate binaries:

```toml
[[bin]]
name = "desktop-app"
required-features = ["desktop"]

[[bin]]
name = "server"
required-features = ["server"]
```

---

## Related Documentation

- [Workspace Structure](./architecture/workspace-structure.md) - Workspace organization
- [LLaMA.cpp Integration](./inference/llama-cpp.md) - Local inference setup
- [Configuration Management](../configs/README.md) - Feature-specific configs
- [Deployment Guide](../deploy/README.md) - Feature requirements per environment

---

## Summary

AgentAsKit's modular feature system provides:

✅ **Flexibility** - Choose only needed features
✅ **Performance** - Smaller binaries with fewer features
✅ **Modularity** - Clear feature dependencies
✅ **Scalability** - From embedded systems to cloud deployment
✅ **Future-Ready** - Experimental features for upcoming capabilities

Use this guide to select the right feature combination for your use case!
