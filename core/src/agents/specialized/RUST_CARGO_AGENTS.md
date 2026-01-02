# Rust/Cargo Sub-Agents

This directory contains 11 specialized Rust/Cargo sub-agents that provide comprehensive Rust ecosystem support for the AgentAsKit platform. These agents are designed to be used by any provider when executing tasks or sessions related to Rust development.

## Overview

All Rust/Cargo sub-agents share the following characteristics:

### Common Outputs
- **Artifacts**: Build outputs, binaries, libraries, documentation, etc.
- **SBOM (Software Bill of Materials)**: Comprehensive package inventories
- **Scores**: Quality, security, and compliance metrics
- **Advisories**: Security vulnerabilities and warnings

### Common Policies
- **MSRV (Minimum Supported Rust Version)**: Version compatibility enforcement
- **Semver**: Semantic versioning validation
- **Export Control**: License and regulatory compliance

## Sub-Agents

### 1. RustCrateScannerAgent
**Purpose**: Discover crates, versions, features; map dependency tree

**Capabilities**:
- Crate discovery and version detection
- Dependency tree mapping and analysis
- Feature detection and compatibility checking
- SBOM generation (CycloneDx, SPDX formats)
- MSRV policy enforcement
- Semver validation
- Advisory scanning

**Key Features**:
- Deep dependency analysis (configurable depth)
- Policy violation detection
- Security scoring
- Compliance reporting

**Usage**:
```rust
let agent = RustCrateScannerAgent::new(None);
let result = agent.scan_workspace(Path::new("/path/to/workspace")).await?;
```

### 2. CargoBuildAgent
**Purpose**: Build/bench/test workflows with caching and EFG-aware parallelism

**Capabilities**:
- Cargo build execution (dev, release, custom profiles)
- Test execution with coverage analysis
- Benchmark execution with performance metrics
- Intelligent build caching
- EFG (Efficient Function Graph) aware parallelism
- Artifact generation and management

**Key Features**:
- Multi-profile builds
- Parallel job execution
- Cache management
- Test coverage reporting
- Benchmark result tracking

**Usage**:
```rust
let agent = CargoBuildAgent::new(None);
let result = agent.build_workspace(Path::new("/path/to/workspace")).await?;
```

### 3. CargoAuditAgent
**Purpose**: Integrate cargo-audit, triage RUSTSEC advisories

**Capabilities**:
- Cargo-audit integration
- RUSTSEC advisory database scanning
- Vulnerability detection and triage
- Security scoring
- Advisory management

**Key Features**:
- Automatic advisory database updates
- Severity-based filtering
- Unmaintained/yanked package detection
- Vulnerability reporting

**Usage**:
```rust
let agent = CargoAuditAgent::new(None);
let result = agent.audit_workspace(Path::new("/path/to/workspace")).await?;
```

### 4. CargoLicenseAgent
**Purpose**: Scan licenses, enforce allow-lists

**Capabilities**:
- License scanning across dependencies
- Allow-list/deny-list enforcement
- Transitive dependency license checking
- Compliance reporting

**Key Features**:
- Configurable license policies
- Multi-license detection
- License violation reporting
- License file discovery

**Usage**:
```rust
let agent = CargoLicenseAgent::new(None);
let result = agent.scan_licenses(Path::new("/path/to/workspace")).await?;
```

### 5. RustClippyAgent
**Purpose**: Clippy linting tiers; autofix common lints

**Capabilities**:
- Clippy integration
- Multi-tier linting (pedantic, recommended, warn, deny)
- Automatic lint fixing
- Code quality analysis
- Style enforcement

**Key Features**:
- Configurable lint levels
- Autofix for common issues
- Custom lint rules
- Detailed error reporting

**Usage**:
```rust
let agent = RustClippyAgent::new(None);
let result = agent.lint_workspace(Path::new("/path/to/workspace")).await?;
```

### 6. RustFmtAgent
**Purpose**: Format code; enforce style policies

**Capabilities**:
- Code formatting with rustfmt
- Style policy enforcement
- Format checking
- Edition-specific formatting

**Key Features**:
- Automatic formatting
- Check-only mode
- Configurable style settings
- Format issue reporting

**Usage**:
```rust
let agent = RustFmtAgent::new(None);
let result = agent.format_workspace(Path::new("/path/to/workspace")).await?;
```

### 7. RustDocAgent
**Purpose**: Generate and publish doc artifacts

**Capabilities**:
- Documentation generation with rustdoc
- Private item documentation
- Documentation publishing
- Documentation coverage analysis

**Key Features**:
- Comprehensive doc generation
- Coverage metrics
- Browser preview
- Warning detection

**Usage**:
```rust
let agent = RustDocAgent::new(None);
let result = agent.generate_docs(Path::new("/path/to/workspace")).await?;
```

### 8. RustFFIAgent
**Purpose**: bindgen/cbindgen pipelines, ABI tests

**Capabilities**:
- Bindgen integration (C -> Rust bindings)
- Cbindgen integration (Rust -> C headers)
- ABI compatibility testing
- Multi-language binding generation

**Key Features**:
- C/C++ binding generation
- ABI stability verification
- Cross-language interop support
- Header file generation

**Usage**:
```rust
let agent = RustFFIAgent::new(None);
let result = agent.generate_bindings(Path::new("/path/to/workspace")).await?;
```

### 9. RustWasmAgent
**Purpose**: wasm-pack + size/perf budgeting, bindings

**Capabilities**:
- WASM compilation with wasm-pack
- Size optimization and budgeting
- Performance budgeting
- JavaScript/TypeScript binding generation
- Multi-target support (web, nodejs, bundler)

**Key Features**:
- Size analysis and optimization
- Performance metrics
- Binding generation
- Target-specific builds

**Usage**:
```rust
let agent = RustWasmAgent::new(None);
let result = agent.build_wasm(Path::new("/path/to/workspace")).await?;
```

### 10. RustCrossAgent
**Purpose**: Cross-compile matrix: musl/aarch64 etc.

**Capabilities**:
- Cross-compilation for multiple targets
- Target matrix management
- Parallel cross-compilation
- Multi-platform artifact generation

**Supported Targets**:
- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl
- x86_64-pc-windows-gnu/msvc
- x86_64-apple-darwin
- aarch64-unknown-linux-gnu/musl
- aarch64-apple-darwin
- armv7-unknown-linux-gnueabihf
- Custom targets

**Key Features**:
- Parallel builds
- Target-specific artifacts
- Cross-compilation caching
- Docker integration support

**Usage**:
```rust
let agent = RustCrossAgent::new(None);
let result = agent.cross_compile(Path::new("/path/to/workspace")).await?;
```

### 11. RustReleaseAgent
**Purpose**: Crate publishing workflow (private/public)

**Capabilities**:
- Crate publishing to crates.io
- Private registry support
- Pre-publish validation
- Version management
- Git tag integration

**Key Features**:
- Dry-run mode
- Pre-publish checks
- Registry configuration
- Version verification
- Documentation publishing

**Usage**:
```rust
let agent = RustReleaseAgent::new(None);
let result = agent.release_crate(Path::new("/path/to/workspace")).await?;
```

## Integration

All Rust/Cargo sub-agents are automatically registered with the SpecializedLayer and can be accessed by name:

```rust
let layer = SpecializedLayer::new().await?;

// Get specific agent
let scanner_id = layer.get_agent_by_name("rust_crate_scanner").await;
let build_id = layer.get_agent_by_name("cargo_build").await;

// List all Rust sub-agents
let agents = layer.list_agent_names().await;
```

## Agent Names

- `rust_crate_scanner` - RustCrateScannerAgent
- `cargo_build` - CargoBuildAgent
- `cargo_audit` - CargoAuditAgent
- `cargo_license` - CargoLicenseAgent
- `rust_clippy` - RustClippyAgent
- `rust_fmt` - RustFmtAgent
- `rust_doc` - RustDocAgent
- `rust_ffi` - RustFFIAgent
- `rust_wasm` - RustWasmAgent
- `rust_cross` - RustCrossAgent
- `rust_release` - RustReleaseAgent

## Configuration

Each agent can be configured with custom settings:

```rust
let config = RustCrateScannerConfig {
    msrv_policy: MsrvPolicy {
        enabled: true,
        minimum_version: "1.70.0".to_string(),
        check_dependencies: true,
        fail_on_violation: true,
    },
    semver_policy: SemverPolicy {
        enforce_semver: true,
        allow_pre_release: false,
        allow_yanked: false,
        max_major_version_delta: Some(2),
    },
    ..Default::default()
};

let agent = RustCrateScannerAgent::new(Some(config));
```

## Testing

All agents include comprehensive unit tests:

```bash
cargo test --package agentaskit-core --lib agents::specialized
```

## Resource Requirements

All Rust/Cargo sub-agents have defined resource requirements:

| Agent | CPU Cores | Memory (MB) | Storage (MB) | Network (Mbps) |
|-------|-----------|-------------|--------------|----------------|
| RustCrateScannerAgent | 2 | 1024 | 2048 | 10 |
| CargoBuildAgent | 4 | 4096 | 10240 | 50 |
| CargoAuditAgent | 1 | 512 | 1024 | 10 |
| CargoLicenseAgent | 1 | 512 | 512 | 5 |
| RustClippyAgent | 2 | 1024 | 512 | 5 |
| RustFmtAgent | 1 | 512 | 256 | 5 |
| RustDocAgent | 2 | 1024 | 2048 | 10 |
| RustFFIAgent | 2 | 1024 | 1024 | 10 |
| RustWasmAgent | 2 | 2048 | 2048 | 20 |
| RustCrossAgent | 4 | 4096 | 10240 | 50 |
| RustReleaseAgent | 2 | 1024 | 2048 | 50 |

## License

Same as the parent project - MIT OR Apache-2.0
