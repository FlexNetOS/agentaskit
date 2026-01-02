# llama.cpp Integration

This integration wires llama.cpp into AgentAsKit behind an optional feature flag and a selector bridge.

## Contents
- `fetch.sh` - clones FlexNetOS llama.cpp fork, llama-cpp-rs bindings, and CMakeRust tool
- `build.sh` - builds llama.cpp using cmake (supports CPU, CUDA, Metal)
- `../../core/src/ai/model_selector_bridge.rs` - selector bridge skeleton (feature-gated)

## Usage
```bash
./fetch.sh
./build.sh
```

## Repositories
- **llama.cpp**: https://github.com/FlexNetOS/llama.cpp.git
- **llama-cpp-rs bindings**: https://github.com/FlexNetOS/llama-cpp-rs.git
- **CMakeRust tool**: https://github.com/Devolutions/CMakeRust.git

## Build System
This integration uses **cmake** instead of make for better cross-platform support and integration with Rust via CMakeRust.

### Build Options
The default build is CPU-only. To enable GPU support:

**CUDA:**
```bash
cd llama.cpp/build
cmake .. -DCMAKE_BUILD_TYPE=Release -DLLAMA_CUDA=ON
cmake --build . -j$(nproc) --config Release
```

**Metal (macOS):**
```bash
cd llama.cpp/build
cmake .. -DCMAKE_BUILD_TYPE=Release -DLLAMA_METAL=ON
cmake --build . -j$(nproc) --config Release
```

## Integration with Rust
The llama-cpp-rs bindings provide Rust FFI bindings to llama.cpp. CMakeRust facilitates building C/C++ dependencies alongside Rust projects.

## Notes
- Do not commit large binaries; use build artifacts locally or CI cache.
- Default workspace build remains unaffected; enable via a feature flag in code when available.
- CMakeRust tool is available in the `CMakeRust/` directory for advanced build scenarios.
