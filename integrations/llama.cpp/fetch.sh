#!/usr/bin/env bash
set -euo pipefail
root_dir="$(cd "$(dirname "$0")" && pwd)"
cd "$root_dir"

# Clone FlexNetOS llama.cpp fork
if [ ! -d llama.cpp ]; then
  git clone https://github.com/FlexNetOS/llama.cpp.git
fi
cd llama.cpp
# Pin to a known good commit (update as needed)
COMMIT=${LLAMACPP_COMMIT:-"master"}
git fetch --all
git checkout "$COMMIT"
echo "Checked out llama.cpp @ $(git rev-parse --short HEAD)"

# Clone llama-cpp-rs bindings
cd "$root_dir"
if [ ! -d llama-cpp-rs ]; then
  git clone https://github.com/FlexNetOS/llama-cpp-rs.git
fi
cd llama-cpp-rs
# Pin to a known good commit (update as needed)
BINDINGS_COMMIT=${LLAMACPP_RS_COMMIT:-"main"}
git fetch --all
git checkout "$BINDINGS_COMMIT"
echo "Checked out llama-cpp-rs @ $(git rev-parse --short HEAD)"

# Clone CMakeRust tool
cd "$root_dir"
if [ ! -d CMakeRust ]; then
  git clone https://github.com/Devolutions/CMakeRust.git
fi
cd CMakeRust
echo "Fetched CMakeRust tool @ $(git rev-parse --short HEAD)"
