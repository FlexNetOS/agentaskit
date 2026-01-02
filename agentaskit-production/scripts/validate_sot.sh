#!/bin/bash
# Validate Source of Truth (SoT) document
# REF: OPS-HOOKS, GOV-SOT-EXEC
# Owner: @platform

set -euo pipefail

SOT_FILE="${1:-core/src/orchestration/sot.md}"
HASH_FILE="${2:-operational_hash/HASHES.txt}"

echo "Validating SoT: $SOT_FILE"
echo "Hash manifest: $HASH_FILE"
echo ""

ERRORS=0
WARNINGS=0

# Check SoT file exists
if [ ! -f "$SOT_FILE" ]; then
    echo "ERROR: SoT file not found: $SOT_FILE"
    exit 1
fi

# Check for required sections
check_section() {
    local section=$1
    if ! grep -q "## $section" "$SOT_FILE"; then
        echo "WARNING: Missing section: ## $section"
        WARNINGS=$((WARNINGS + 1))
    else
        echo "✓ Found section: $section"
    fi
}

echo "=== Checking Required Sections ==="
check_section "Executed Tasks"
check_section "Decisions"
check_section "Artifacts"
echo ""

# Check for evidence links
echo "=== Checking Evidence Links ==="
EVIDENCE_COUNT=$(grep -c '\[Evidence\]' "$SOT_FILE" || echo "0")
echo "Found $EVIDENCE_COUNT evidence links"

if [ "$EVIDENCE_COUNT" -lt 1 ]; then
    echo "WARNING: No evidence links found"
    WARNINGS=$((WARNINGS + 1))
fi
echo ""

# Verify hashes exist
echo "=== Checking Hash Manifest ==="
if [ -f "$HASH_FILE" ]; then
    HASH_COUNT=$(wc -l < "$HASH_FILE")
    echo "Found $HASH_COUNT hash entries"

    # Sample hash verification
    echo "Sample entries:"
    head -3 "$HASH_FILE" | while read -r line; do
        echo "  $line"
    done
else
    echo "WARNING: Hash manifest not found"
    WARNINGS=$((WARNINGS + 1))
fi
echo ""

# Check for stale entries (older than 90 days)
echo "=== Checking Entry Freshness ==="
NINETY_DAYS_AGO=$(date -d '90 days ago' +%Y-%m-%d 2>/dev/null || date -v-90d +%Y-%m-%d 2>/dev/null || echo "2025-07-01")
STALE_ENTRIES=$(grep -E '^\|\s*\d{4}-\d{2}-\d{2}' "$SOT_FILE" | \
    awk -F'|' '{print $2}' | \
    while read -r date; do
        if [[ "$date" < "$NINETY_DAYS_AGO" ]]; then
            echo "$date"
        fi
    done | wc -l || echo "0")

if [ "$STALE_ENTRIES" -gt 0 ]; then
    echo "INFO: $STALE_ENTRIES entries older than 90 days (may need review)"
else
    echo "✓ All entries are recent"
fi
echo ""

# Summary
echo "=== Validation Summary ==="
echo "Errors: $ERRORS"
echo "Warnings: $WARNINGS"

if [ $ERRORS -gt 0 ]; then
    echo ""
    echo "VALIDATION FAILED"
    exit 1
fi

if [ $WARNINGS -gt 0 ]; then
    echo ""
    echo "VALIDATION PASSED with warnings"
    exit 0
fi

echo ""
echo "VALIDATION PASSED"
exit 0
