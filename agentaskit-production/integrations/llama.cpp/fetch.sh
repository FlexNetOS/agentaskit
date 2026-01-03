#!/bin/bash
# Fetch and build llama.cpp
set -e

LLAMA_VERSION="${LLAMA_VERSION:-master}"
VENDOR_DIR="$(dirname "$0")/vendor"

echo "=== Fetching llama.cpp ==="

if [ -d "$VENDOR_DIR" ] && [ -d "$VENDOR_DIR/.git" ]; then
    echo "Updating existing llama.cpp..."
    cd "$VENDOR_DIR"
    git fetch origin
    git checkout "$LLAMA_VERSION"
    git pull origin "$LLAMA_VERSION" || true
else
    echo "Cloning llama.cpp..."
    git clone https://github.com/ggerganov/llama.cpp.git "$VENDOR_DIR"
    cd "$VENDOR_DIR"
    git checkout "$LLAMA_VERSION"
fi

echo "=== Building llama.cpp ==="

# Detect CUDA
if command -v nvcc &> /dev/null; then
    echo "CUDA detected, building with GPU support..."
    make LLAMA_CUDA=1 -j$(nproc)
else
    echo "Building CPU-only..."
    make -j$(nproc)
fi

echo "=== Build Complete ==="
echo "Binary: $VENDOR_DIR/main"
ls -la "$VENDOR_DIR/main" 2>/dev/null || echo "Note: main binary may have different name"
