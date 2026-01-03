#!/bin/bash
# XDG Compliance Audit Tool
# REF: ADR-0005 Modern Tooling Strategy
# Wraps xdg-ninja or provides fallback audit

set -euo pipefail

OUTPUT_DIR="${1:-tools/analysis}"
OUTPUT_FILE="$OUTPUT_DIR/xdg_audit.json"
TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)

mkdir -p "$OUTPUT_DIR"

echo "🔍 Running XDG compliance audit..."
echo ""

# Check if xdg-ninja is available
if command -v xdg-ninja &> /dev/null; then
    echo "Using xdg-ninja..."
    xdg-ninja --json > "$OUTPUT_FILE"
else
    echo "xdg-ninja not found, running manual audit..."

    # Manual XDG audit
    cat > "$OUTPUT_FILE" << JSONEOF
{
  "timestamp": "$TIMESTAMP",
  "tool": "manual-audit",
  "xdg_paths": {
    "XDG_CONFIG_HOME": "${XDG_CONFIG_HOME:-$HOME/.config}",
    "XDG_DATA_HOME": "${XDG_DATA_HOME:-$HOME/.local/share}",
    "XDG_CACHE_HOME": "${XDG_CACHE_HOME:-$HOME/.cache}",
    "XDG_STATE_HOME": "${XDG_STATE_HOME:-$HOME/.local/state}"
  },
  "violations": [
JSONEOF

    # Check for common XDG violators in $HOME
    CHECK_FILES=(
        ".bash_history:HISTFILE:XDG_STATE_HOME/bash/history"
        ".bashrc:N/A:Standard location, acceptable"
        ".cargo:CARGO_HOME:XDG_DATA_HOME/cargo"
        ".rustup:RUSTUP_HOME:XDG_DATA_HOME/rustup"
        ".npm:npm config:XDG_DATA_HOME/npm"
        ".node_repl_history:NODE_REPL_HISTORY:XDG_STATE_HOME/node/history"
        ".python_history:PYTHONSTARTUP:XDG_STATE_HOME/python/history"
        ".local/share:N/A:XDG compliant"
        ".config:N/A:XDG compliant"
        ".cache:N/A:XDG compliant"
    )

    # Build array of violations
    VIOLATIONS_JSON=""
    FIRST=true
    for check in "${CHECK_FILES[@]}"; do
        IFS=':' read -r file var fix <<< "$check"
        if [ -e "$HOME/$file" ]; then
            if [[ "$fix" != *"compliant"* && "$fix" != *"acceptable"* ]]; then
                if [ "$FIRST" = false ]; then
                    VIOLATIONS_JSON+=","
                fi
                FIRST=false
                VIOLATIONS_JSON+="
    {
      \"file\": \"$HOME/$file\",
      \"variable\": \"$var\",
      \"suggested_fix\": \"$fix\"
    }"
            fi
        fi
    done

    cat >> "$OUTPUT_FILE" << ENTRYEOF
$VIOLATIONS_JSON
ENTRYEOF

    cat >> "$OUTPUT_FILE" << JSONEOF
  ],
  "recommendations": [
    "Set CARGO_HOME=\$XDG_DATA_HOME/cargo",
    "Set RUSTUP_HOME=\$XDG_DATA_HOME/rustup",
    "Set HISTFILE=\$XDG_STATE_HOME/bash/history",
    "Configure npm: npm config set prefix \$XDG_DATA_HOME/npm"
  ]
}
JSONEOF
fi

echo ""
echo "✓ Audit complete: $OUTPUT_FILE"
echo ""

# Display summary
if command -v jq &> /dev/null; then
    VIOLATION_COUNT=$(jq '.violations | length' "$OUTPUT_FILE" 2>/dev/null || echo "N/A")
    echo "Violations found: $VIOLATION_COUNT"
else
    echo "Install jq to see violation count"
fi
