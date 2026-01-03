# llama.cpp Integration

This directory contains the integration scaffolding for [llama.cpp](https://github.com/ggerganov/llama.cpp), a high-performance LLM inference library.

## Purpose

Provides local LLM inference capabilities for:
- Model selection and routing
- On-device inference fallback
- Performance-critical inference paths

## Setup

### Option 1: Git Submodule (Recommended)
```bash
git submodule add https://github.com/ggerganov/llama.cpp.git integrations/llama.cpp/vendor
cd integrations/llama.cpp/vendor
make -j
```

### Option 2: Fetch Script
```bash
./integrations/llama.cpp/fetch.sh
```

## Build

```bash
# Build with CUDA support (if available)
cd integrations/llama.cpp/vendor
make LLAMA_CUDA=1 -j

# Build CPU-only
make -j
```

## Integration Points

### Model Selector Bridge
The `core/src/ai/model_selector_bridge.rs` provides the Rust interface:

```rust
#[cfg(feature = "llama-cpp")]
pub fn select_local_model(requirements: &ModelRequirements) -> Option<LocalModel> {
    // Implementation uses llama.cpp bindings
}
```

### Feature Flag
Enable in Cargo.toml:
```toml
[features]
llama-cpp = ["llama-cpp-rs"]
```

## CI Integration

The llama.cpp build is gated behind an opt-in CI flag to avoid breaking default builds:

```yaml
jobs:
  build-with-llama:
    if: contains(github.event.head_commit.message, '[llama-cpp]')
```

## References

- [REF: WORKFLOW-012] - Integration task
- [REF: SUBJ-LLAMACPP] - Subject inbox item
