# LLaMA.cpp Integration Guide

## Overview

LLaMA.cpp provides fast, efficient local LLM (Large Language Model) inference for AgentAsKit. This guide covers setup, configuration, model management, and integration.

## Table of Contents

- [Quick Start](#quick-start)
- [Setup and Installation](#setup-and-installation)
- [Model Management](#model-management)
- [Feature Flag Configuration](#feature-flag-configuration)
- [Usage Examples](#usage-examples)
- [Performance Tuning](#performance-tuning)
- [GPU Acceleration](#gpu-acceleration)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

### 1. Enable the Feature

Add the `inference-llama` feature to your build:

```bash
cargo build --features "inference-llama"
```

### 2. Initialize Submodule

```bash
git submodule update --init integrations/llama.cpp
```

### 3. Build llama.cpp

```bash
cd integrations/llama.cpp
./fetch.sh    # Clone necessary repositories
./build.sh    # Build with cmake
```

### 4. Run with Local Models

```bash
# Set model path environment variable
export LLAMA_CPP_PATH="./integrations/llama.cpp/main"

# Run AgentAsKit with inference enabled
cargo run --features "inference-llama" -- --help
```

---

## Setup and Installation

### Prerequisites

**System Requirements:**
- C++ compiler (gcc, clang, or MSVC)
- CMake 3.13+
- Git
- Rust 1.70+

**Package Installation:**

#### Ubuntu/Debian
```bash
sudo apt-get install build-essential cmake git
```

#### macOS
```bash
# Using Homebrew
brew install cmake
# Xcode command line tools (if not installed)
xcode-select --install
```

#### Windows
```bash
# Using MSVC (via Visual Studio or Visual Studio Build Tools)
# Download from: https://visualstudio.microsoft.com/

# Or use Scoop
scoop install cmake
```

### Step-by-Step Setup

#### Step 1: Clone llama.cpp Submodule

```bash
cd /home/user/agentaskit

# Initialize the submodule
git submodule update --init integrations/llama.cpp

# Verify it's initialized
ls integrations/llama.cpp/
```

#### Step 2: Run Fetch Script

The fetch script clones required dependencies:

```bash
cd integrations/llama.cpp

# Make script executable (if needed)
chmod +x fetch.sh

# Run fetch script
./fetch.sh
```

**What it downloads:**
- `llama.cpp` repository
- `llama-cpp-rs` Rust bindings
- `CMakeRust` build integration

#### Step 3: Build llama.cpp

```bash
# Make build script executable
chmod +x build.sh

# Build (CPU only, default)
./build.sh

# Or build with specific backend
./build.sh cpu      # CPU-only build
./build.sh cuda     # NVIDIA CUDA support
./build.sh metal    # Apple Metal (macOS)
./build.sh rocm     # AMD ROCm support
```

**Build verification:**
```bash
# Check if main binary exists
ls -la main

# Test the build
./main --help
```

#### Step 4: Verify Rust Integration

```bash
# Return to project root
cd /home/user/agentaskit

# Verify llama-cpp-rs bindings compile
cargo build --features "inference-llama" --lib
```

---

## Model Management

### Obtaining GGUF Models

GGUF is the quantized format used by llama.cpp. Models are available from various sources.

#### Recommended Model Sources

**Hugging Face:**
- [TheBloke's Quantized Models](https://huggingface.co/TheBloke) - Excellent selection of quantized models
- [Mistral Community](https://huggingface.co/mistralai)
- [Meta Llama](https://huggingface.co/meta-llama)

**Model Zoos:**
- [ollama.ai](https://ollama.ai) - Pre-quantized models
- [HuggingFace GGUF Collections](https://huggingface.co/search/full-text?q=GGUF)

#### Downloading Models

**Using Hugging Face CLI:**

```bash
# Install huggingface-hub
pip install huggingface-hub

# Download a model
huggingface-cli download TheBloke/Mistral-7B-Instruct-v0.1-GGUF mistral-7b-instruct-v0.1.Q4_K_M.gguf --local-dir ./models
```

**Using wget/curl:**

```bash
# Create models directory
mkdir -p models

# Download Mistral 7B Instruct (Q4_K_M - 4-bit)
wget -O models/mistral-7b-instruct.gguf \
  https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.1-GGUF/resolve/main/mistral-7b-instruct-v0.1.Q4_K_M.gguf
```

### Recommended Models for Different Use Cases

#### General Purpose (Balanced)
- **Mistral 7B Instruct** (Q4_K_M)
  - Size: 4.3 GB
  - Speed: Fast
  - Quality: High
  - VRAM: 6-8 GB

- **Llama 2 13B Chat** (Q4_K_M)
  - Size: 7.4 GB
  - Speed: Moderate
  - Quality: Very High
  - VRAM: 8-10 GB

#### Small/Fast (Resource Constrained)
- **Phi 2.7B** (Q4_K_M)
  - Size: 1.6 GB
  - Speed: Very Fast
  - Quality: Good
  - VRAM: 4-6 GB

- **TinyLlama 1.1B** (Q4_K_M)
  - Size: 650 MB
  - Speed: Extremely Fast
  - Quality: Fair
  - VRAM: 2-3 GB

#### High Quality (Large GPU Required)
- **Llama 2 70B Chat** (Q2_K)
  - Size: 26 GB
  - Speed: Slow
  - Quality: Excellent
  - VRAM: 24-40 GB

### Model Organization

```
./models/
├── mistral-7b-instruct.gguf
├── llama-2-13b-chat.gguf
└── phi-2.7b.gguf
```

---

## Feature Flag Configuration

### Add Feature Flag to Cargo.toml

The `inference-llama` feature is already defined in `core/Cargo.toml`:

```toml
[features]
inference-llama = []
```

### Conditional Compilation

Code using llama.cpp is feature-gated:

```rust
#[cfg(feature = "inference-llama")]
pub mod llama_bridge {
    // llama.cpp bindings and integration code
    // This module only compiles when inference-llama is enabled
}
```

### Using Gated Code

**In application code:**

```rust
#[cfg(feature = "inference-llama")]
use agentaskit_core::ai::llama_bridge::LlamaInference;

#[cfg(feature = "inference-llama")]
async fn use_local_inference() -> Result<String> {
    let inference = LlamaInference::new("models/mistral-7b-instruct.gguf")?;
    let response = inference.complete("What is AI?").await?;
    Ok(response)
}
```

---

## Usage Examples

### Basic Inference

```rust
use agentaskit_core::ai::llama_bridge::LlamaInference;

#[tokio::main]
async fn main() -> Result<()> {
    // Create inference instance
    let llama = LlamaInference::new(
        "models/mistral-7b-instruct.gguf"
    )?;

    // Generate completion
    let prompt = "What is machine learning?";
    let response = llama.complete(prompt).await?;

    println!("Response: {}", response);
    Ok(())
}
```

### With Configuration

```rust
use agentaskit_core::ai::llama_bridge::{LlamaInference, InferenceConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = InferenceConfig {
        model_path: "models/mistral-7b-instruct.gguf".to_string(),
        n_gpu_layers: 40,        // GPU layers (increase for GPU acceleration)
        n_threads: 8,            // CPU threads
        context_size: 2048,      // Context window size
        batch_size: 512,         // Batch size
    };

    let llama = LlamaInference::with_config(config)?;

    let response = llama.complete("Explain quantum computing").await?;
    println!("{}", response);

    Ok(())
}
```

### Streaming Responses

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let llama = LlamaInference::new(
        "models/mistral-7b-instruct.gguf"
    )?;

    // Stream tokens as they're generated
    let mut stream = llama.complete_stream(
        "Write a short poem about AI"
    ).await?;

    while let Some(token) = stream.next().await {
        print!("{}", token?);
        io::stdout().flush()?;
    }

    println!();
    Ok(())
}
```

### Multi-Model Switching

```rust
use agentaskit_core::ai::model_selector_bridge::ModelSelector;

#[tokio::main]
async fn main() -> Result<()> {
    let mut selector = ModelSelector::new();

    // Register available models
    selector.register("fast", "models/phi-2.7b.gguf")?;
    selector.register("balanced", "models/mistral-7b-instruct.gguf")?;
    selector.register("quality", "models/llama-2-13b-chat.gguf")?;

    // Use specific model
    let response = selector.complete("fast", "Hello").await?;
    println!("{}", response);

    Ok(())
}
```

---

## Performance Tuning

### Key Parameters

#### n_gpu_layers (GPU Acceleration)
- **Effect:** Number of transformer layers to run on GPU
- **Range:** 0 to total layers (depends on model)
- **Guidance:**
  - `0`: CPU only (slower but uses less VRAM)
  - `20-40`: Balanced (some GPU acceleration)
  - `80+`: Full GPU (fastest, needs more VRAM)

**Example:**
```rust
let config = InferenceConfig {
    n_gpu_layers: 35,  // Put 35 layers on GPU
    ..Default::default()
};
```

#### n_threads (CPU Threads)
- **Effect:** Number of CPU threads for computation
- **Guidance:**
  - Typical: Match CPU core count
  - Example: 8-core CPU = 8 threads
  - Use `num_cpus::get()` to auto-detect

#### context_size (Context Window)
- **Effect:** Maximum tokens in conversation history
- **Guidance:**
  - Small (512-1024): Fast, limited context
  - Medium (2048-4096): Balanced
  - Large (8192+): Slow, good context retention
- **VRAM Impact:** Larger context = more VRAM

#### batch_size (Processing Batch)
- **Effect:** Tokens processed in parallel
- **Guidance:**
  - Smaller (128-256): Lower VRAM
  - Larger (512-1024): Faster inference

### Optimization Example

```rust
// For fast, resource-constrained inference
let config = InferenceConfig {
    n_gpu_layers: 0,        // CPU only
    n_threads: 4,           // Few threads
    context_size: 512,      // Small context
    batch_size: 128,        // Small batch
};

// For maximum performance
let config = InferenceConfig {
    n_gpu_layers: 80,       // Full GPU
    n_threads: 16,          // All CPU cores
    context_size: 4096,     // Large context
    batch_size: 512,        // Large batch
};
```

---

## GPU Acceleration

### NVIDIA CUDA

**Requirements:**
- NVIDIA GPU (Compute Capability 3.5+)
- CUDA Toolkit 11.0+
- cuDNN (optional, for better performance)

**Build with CUDA:**

```bash
cd integrations/llama.cpp
./build.sh cuda
```

**Enable GPU in Code:**

```rust
let config = InferenceConfig {
    n_gpu_layers: 50,  // Use GPU for 50 layers
    ..Default::default()
};
```

### Apple Metal (macOS)

**Automatic on macOS** with compatible GPU.

**Build with Metal:**

```bash
cd integrations/llama.cpp
./build.sh metal
```

**Metal is automatically detected and used:**

```rust
// Metal acceleration is used automatically on macOS
let config = InferenceConfig {
    n_gpu_layers: 40,  // Metal will accelerate these
    ..Default::default()
};
```

### AMD ROCm

**Requirements:**
- AMD GPU (RDNA or CDNA architecture)
- ROCm Toolkit 5.0+

**Build with ROCm:**

```bash
cd integrations/llama.cpp
./build.sh rocm
```

---

## Troubleshooting

### Issue: Model File Not Found

**Error:** `Model file not found: models/model.gguf`

**Solutions:**
1. Verify model path: `ls -la models/model.gguf`
2. Use absolute path: `/full/path/to/model.gguf`
3. Set environment variable: `export LLAMA_MODEL_PATH=./models`

### Issue: Out of Memory (OOM)

**Error:** `CUDA/Metal out of memory`

**Solutions:**
1. Reduce `n_gpu_layers` (move more computation to CPU)
2. Reduce `context_size` (smaller conversation history)
3. Reduce `batch_size` (process fewer tokens at once)
4. Use smaller model (e.g., 7B instead of 13B)

**Example:**
```rust
let config = InferenceConfig {
    n_gpu_layers: 10,      // Reduced from 40
    context_size: 1024,    // Reduced from 4096
    batch_size: 256,       // Reduced from 512
    ..Default::default()
};
```

### Issue: Slow Inference

**Solutions:**
1. Increase `n_gpu_layers` if GPU available
2. Increase `n_threads` if CPU-bound
3. Reduce `context_size` if not needed
4. Use smaller model variant (Q4 instead of Q6, 7B instead of 13B)

### Issue: Feature Not Compiled

**Error:** `module llama_bridge not found`

**Solution:** Rebuild with feature:
```bash
cargo build --features "inference-llama"
```

### Issue: CUDA Not Detected

**Solutions:**
1. Verify CUDA installation: `nvidia-smi`
2. Rebuild llama.cpp: `cd integrations/llama.cpp && ./build.sh cuda`
3. Check CUDA paths in environment

### Issue: Build Fails

**Solutions:**
1. Update CMake: `cmake --version` (need 3.13+)
2. Update compilers: `gcc --version`, `clang --version`
3. Clean and rebuild:
   ```bash
   cd integrations/llama.cpp
   rm -rf build cmake_build
   ./build.sh
   ```

---

## Environment Variables

### Model Loading

```bash
# Set default model path
export LLAMA_MODEL_PATH="./models/mistral-7b.gguf"

# Set llama.cpp main executable path
export LLAMA_CPP_PATH="./integrations/llama.cpp/main"
```

### Performance Tuning

```bash
# Number of GPU layers (CUDA/Metal)
export LLAMA_N_GPU_LAYERS=40

# Number of threads
export LLAMA_N_THREADS=8

# Context size
export LLAMA_CONTEXT_SIZE=2048

# Batch size
export LLAMA_BATCH_SIZE=512
```

### Debugging

```bash
# Enable verbose logging
export LLAMA_VERBOSE=1

# Enable CUDA debug output
export CUDA_LAUNCH_BLOCKING=1
```

---

## Performance Benchmarks

### Relative Speeds (tokens/second)

| Model | Size | CPU (8t) | GPU (40%) | GPU (full) |
|-------|------|----------|-----------|-----------|
| Phi 2.7B (Q4) | 1.6GB | 15 tok/s | 50 tok/s | 80 tok/s |
| Mistral 7B (Q4) | 4.3GB | 8 tok/s | 40 tok/s | 70 tok/s |
| Llama 2 13B (Q4) | 7.4GB | 4 tok/s | 20 tok/s | 50 tok/s |
| Llama 2 70B (Q2) | 26GB | 1 tok/s | 8 tok/s | 25 tok/s |

*Benchmarks are approximate and depend on hardware. GPU performance requires NVIDIA CUDA, Apple Metal, or AMD ROCm.*

---

## Related Documentation

- [Feature Flags](../features.md#inference-llama) - inference-llama feature documentation
- [Workspace Structure](../architecture/workspace-structure.md) - Directory organization
- [Configuration Management](../../configs/README.md) - Config system
- [AGI Framework](../../README.md) - Main documentation

---

## Summary

LLaMA.cpp provides fast, efficient local LLM inference for AgentAsKit:

✅ **Zero API costs** - Run models locally
✅ **Privacy** - No data sent to external services
✅ **Offline** - Works without internet connection
✅ **Customizable** - Choose models for your needs
✅ **GPU accelerated** - CUDA, Metal, ROCm support
✅ **Production ready** - Stable, well-tested implementation

Start with the [Quick Start](#quick-start) and refer to this guide for advanced configuration!
