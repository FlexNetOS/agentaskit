#!/usr/bin/env bash
set -euo pipefail
root_dir="$(cd "$(dirname "$0")" && pwd)"
conf="$root_dir/config/models.yaml"

# Prefer env vars for offline use
primary_path="${LLAMACPP_PRIMARY_3B:-}"
secondary_path="${LLAMACPP_SECONDARY_7B:-}"
count="${LLAMACPP_STACKS_COUNT:-}"
prompt="${LLAMACPP_PROMPT:-}"
threads="${LLAMACPP_THREADS:-}"

# If env not set, try yq only if available
if [[ -z "$primary_path" || -z "$secondary_path" || -z "$count" || -z "$prompt" || -z "$threads" ]]; then
  if command -v yq >/dev/null 2>&1; then
    primary_path=${primary_path:-$(yq -r '.models.primary_3b.path' "$conf")}
    secondary_path=${secondary_path:-$(yq -r '.models.secondary_7b.path' "$conf")}
    count=${count:-$(yq -r '.stacks.count' "$conf")}
    prompt=${prompt:-$(yq -r '.stacks.prompt' "$conf")}
    threads=${threads:-$(yq -r '.stacks.threads' "$conf")}
  fi
fi

# Default sensible fallbacks
count=${count:-1}
prompt=${prompt:-ping}
threads=${threads:-4}

exe="$root_dir/llama.cpp/main"
if [ ! -x "$exe" ]; then exe="$root_dir/llama.cpp/build/bin/main"; fi
if [ ! -x "$exe" ]; then echo "llama.cpp main executable not found; skip" >&2; exit 0; fi

for i in $(seq 1 "$count"); do
  if [ -n "${primary_path}" ] && [ -f "$primary_path" ]; then
    ("$exe" -m "$primary_path" -p "$prompt" -t "$threads" &)
  fi
  if [ -n "${secondary_path}" ] && [ -f "$secondary_path" ]; then
    ("$exe" -m "$secondary_path" -p "$prompt" -t "$threads" &)
  fi
done
wait
