# Build System Documentation

AgentAskit uses **Cargo** (Rust's build system) as the primary build tool.

## Directory Structure

```
agentaskit/
├── .cargo/
│   └── config.toml       # Cargo build configuration
├── target/               # Build output (auto-created by Cargo)
│   ├── debug/           # Debug builds
│   └── release/         # Release builds
├── Cargo.toml           # Root workspace definition
├── Cargo.lock           # Dependency lockfile
├── pixi.toml            # Pixi package manager config
├── .mise.toml           # mise tool version manager
└── core/                # Main library crate
    ├── Cargo.toml
    ├── build.rs         # Build script (protobuf generation)
    └── src/
```

## Quick Start

```bash
# Using mise (recommended)
mise run build

# Using pixi
pixi run build

# Using cargo directly
cargo build --release
```

## Build Commands

| Command | Description |
|---------|-------------|
| `cargo build` | Development build |
| `cargo build --release` | Optimized release build |
| `cargo test --all` | Run all tests |
| `cargo clippy` | Run linter |
| `cargo fmt` | Format code |
| `cargo doc --open` | Generate and view docs |

## Build Acceleration with sccache

For faster rebuilds, enable [sccache](https://github.com/mozilla/sccache):

```bash
# Install sccache
cargo install sccache

# Or via pixi (recommended)
pixi install sccache

# Enable in your shell
export RUSTC_WRAPPER=sccache

# Or use the cached build task
pixi run build-cached
```

## Cross-Platform Builds

### Linux (Static Binary)
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

### WebAssembly
```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

### macOS (Universal Binary)
```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
# Combine with lipo for universal binary
```

### Windows
```bash
# From Linux/macOS (cross-compile)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Build Profiles

Defined in `Cargo.toml`:

| Profile | Optimization | Debug Info | Use Case |
|---------|-------------|------------|----------|
| `dev` | None | Full | Development |
| `release` | Max (thin LTO) | None | Production |
| `release-lto` | Max (fat LTO) | None | Maximum performance |
| `bench` | Max | Full | Benchmarking |

## Workspaces

The project uses Cargo workspaces:

**Root Workspace** (`/Cargo.toml`):
- `core` - Main library
- `shared` - Shared utilities

**Production Workspace** (`/agentaskit-production/Cargo.toml`):
- `core` - Production core
- `unified_execution/core` - Execution engine
- `unified_execution/wasm_host` - WASM runtime
- `shared` - Shared types
- `services` - Service implementations

## Protobuf/gRPC

The `core/build.rs` script automatically compiles `.proto` files if they exist:

```rust
// Looks for proto/agentaskit.proto
tonic_build::compile(&["proto/agentaskit.proto"], &["proto"])
```

## Why Not CMake?

This is a **pure Rust project**. CMake is not needed because:

1. Cargo handles all Rust compilation
2. No C/C++ code in the main project
3. External tools (pixi examples) have their own CMakeLists.txt but are not part of the main build

The `pixi.toml` previously listed cmake as a dependency - this was incorrect and has been removed.

## Troubleshooting

### Slow Builds
1. Enable sccache: `export RUSTC_WRAPPER=sccache`
2. Use incremental compilation: Already enabled in `.cargo/config.toml`
3. Use thin LTO: Default in release profile

### Missing Dependencies
```bash
# Check for missing system libraries
cargo build 2>&1 | grep "error: could not find"

# On Linux (Debian/Ubuntu)
sudo apt-get install build-essential pkg-config
```

### Clean Rebuild
```bash
cargo clean
cargo build --release
```

## CI/CD Build

See `.github/workflows/` for CI configuration. The build uses:

1. Cached dependencies (`~/.cargo/registry`)
2. sccache for build caching
3. Matrix builds for Linux/macOS/Windows

## Integrations

### Building aichat (AI CLI)

```bash
cd integrations/aichat
cargo build --release
# Binary at: target/release/aichat
```

### Building llama.cpp (Local Inference)

```bash
cd integrations/llama.cpp
make -j$(nproc)
# Binary at: main
```

### Installing claude-flow (Orchestration)

```bash
cd integrations/claude-flow
npm install
# Or use npx directly
npx claude-flow --help
```

### Gateway Configuration

Gateway configs are in `configs/agentgateway/`:

| Config | Purpose |
|--------|---------|
| `local.yaml` | Local development |
| `integrated.yaml` | Full integration with aichat, claude-flow, llama.cpp |

See [AGENT_GATEWAY_INTEGRATION.md](AGENT_GATEWAY_INTEGRATION.md) for architecture details.
