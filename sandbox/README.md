# Sandbox Directory - Tri-Sandbox Parallel Execution

**Purpose:** Isolated execution environment for parallel A/B/C testing and evolutionary merge to Model D.

## Directory Structure

```
sandbox/
├── inputs/          # Input data for all sandbox runs
├── outputs/         # Combined outputs from A/B/C runs
├── tri-sandbox/     # Three parallel sandbox environments
│   ├── A/           # Sandbox A - Variant 1
│   ├── B/           # Sandbox B - Variant 2
│   ├── C/           # Sandbox C - Variant 3
│   └── unifier/     # Merge logic (A/B/C → D)
│       └── merge.py
├── parent/          # Model D output location
│   ├── model-D/     # Merged model
│   └── fitness-report.json
└── README.md        # This file
```

## Usage

### 1. Prepare Inputs

Place input data in `inputs/` directory:

```bash
cp your-data.json sandbox/inputs/
```

### 2. Run Tri-Sandbox Execution

```bash
make tri-run
```

This executes:
- **Sandbox A:** First variant/strategy
- **Sandbox B:** Second variant/strategy  
- **Sandbox C:** Third variant/strategy

### 3. Merge to Model D

```bash
make merge
```

This:
- Analyzes A/B/C outputs
- Applies evolutionary merge heuristic
- Produces unified Model D
- Generates fitness report

### 4. Review Results

```bash
# Check merged model
ls -la parent/model-D/

# Review fitness metrics
cat parent/fitness-report.json
```

## Tri-Sandbox Configuration

Each sandbox (A/B/C) can have:
- **run.sh** - Execution script
- **config.json** - Sandbox-specific configuration
- **env** - Environment variables
- **outputs/** - Sandbox-specific results

## Merge Strategy

The `unifier/merge.py` uses:
- **Majority voting** for deterministic outputs
- **Fitness scoring** for competing results
- **Domain metrics** (customizable)

## Environment Variables

- `FLEX_SANDBOX_MODE` - Sandbox isolation level (strict|permissive)
- `FLEX_PREOPEN_DIR` - Directory to preopen at /cap for WASM connectors
- `FLEX_TRI_TIMEOUT` - Max execution time per sandbox (seconds)

## Best Practices

1. **Isolation:** Each sandbox runs independently with no cross-contamination
2. **Reproducibility:** Use fixed seeds and deterministic inputs
3. **Metrics:** Define clear fitness criteria in merge.py
4. **Cleanup:** Clear outputs/ between runs for fresh execution

## Troubleshooting

**Problem:** Sandbox A/B/C produce no output
- **Solution:** Check run.sh scripts in tri-sandbox/{A,B,C}/ have execute permissions

**Problem:** Merge fails with no fitness data
- **Solution:** Ensure all sandboxes completed successfully before merging

**Problem:** Model D differs between runs
- **Solution:** Enable deterministic mode with fixed random seeds

## Related Documentation

- [Agent Task Lifecycle SOP](../agentask.sop)
- [Makefile Targets](../Makefile) - See tri-run, merge targets
- [Merge Implementation](tri-sandbox/unifier/merge.py)

---

**Generated:** 2025-01-05 | **Version:** 1.0 | **Status:** Production Ready
