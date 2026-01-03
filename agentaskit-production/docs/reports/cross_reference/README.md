# Cross-Reference Analysis Reports

This directory contains cross-reference analysis reports comparing production code against archive versions (V2-V7).

## Purpose

The cross-reference analysis helps maintain code integrity by:

1. **Lineage Tracking**: Tracing file evolution across versions
2. **Duplicate Detection**: Identifying redundant code
3. **Gap Analysis**: Finding missing production components
4. **Integrity Verification**: Ensuring no regressions

## Running Analysis

```bash
python tools/analysis/cross_reference.py \
  --production-dir agentaskit-production \
  --archive-dir archive \
  --output-dir docs/reports/cross_reference/artifacts
```

## Evidence Requirements

Per [REF: WORKFLOW-006], this analysis produces deterministic outputs with CI artifacts.
