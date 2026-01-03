#!/usr/bin/env bash
set -euo pipefail
root_dir="$(cd "$(dirname "$0")" && pwd)"
cd "$root_dir/llama.cpp"

# Use cmake instead of make for building
# Check if cmake is available
if ! command -v cmake &> /dev/null; then
    echo "Error: cmake is not installed. Please install cmake first."
    exit 1
fi

# Create build directory
mkdir -p build
cd build

# Configure with cmake
# Basic CPU build - extend for CUDA/Metal as needed
cmake .. \
  -DCMAKE_BUILD_TYPE=Release \
  -DBUILD_SHARED_LIBS=ON

# Build with all available cores
cmake --build . -j$(nproc) --config Release

echo "Successfully built llama.cpp with cmake"
echo "Build artifacts are in: $(pwd)"
