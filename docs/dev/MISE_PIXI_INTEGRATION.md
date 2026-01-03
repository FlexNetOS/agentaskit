# mise + pixi Integration Strategy

## Problem Identified

**Issue:** `.mise.toml` existed but `mise` binary was not installed
**Root Cause:** mise was optional but not provided by any package manager
**Impact:** Developers couldn't use `mise run <task>` commands

## Solution

**Strategy:** Use pixi to provide mise, creating a unified toolchain

```toml
# pixi.toml
[dependencies]
mise = "*"  # ← mise is now provided by pixi!
```

## How It Works

### 1. **Pixi Provides Everything**
```bash
pixi shell              # Activates environment with mise installed
pixi run build          # Uses pixi tasks
mise run build          # Uses mise tasks (mise provided by pixi!)
cargo build             # Direct cargo (provided by pixi)
```

### 2. **Task Equivalence**
Both `pixi.toml` and `.mise.toml` define the same tasks:

| Task | pixi.toml | .mise.toml | Effect |
|------|-----------|------------|--------|
| `build` | ✅ | ✅ | Same: `cargo build --release` |
| `test` | ✅ | ✅ | Same: `cargo test --all` |
| `lint` | ✅ | ✅ | Same: clippy + fmt |

### 3. **Developer Choice**
Developers can use either:
- `pixi run <task>` - Simpler, newer tool
- `mise run <task>` - Familiar to mise users

Both work because:
1. pixi provides the mise binary
2. Both configs define equivalent tasks
3. Both use the same underlying tools (cargo, nu, etc.)

## Why Keep Both?

### mise.toml Benefits
- ✅ Familiar to developers already using mise
- ✅ Rich task dependency system
- ✅ Environment variable management
- ✅ Backward compatibility

### pixi.toml Benefits
- ✅ Better cross-platform support (conda-forge packages)
- ✅ Lockfile for reproducibility
- ✅ Integrated with CI/CD (prefix-dev/setup-pixi action)
- ✅ Better Windows support

## Recommended Workflow

### For New Developers
```bash
# Setup
pixi install           # Install all dependencies
pixi shell             # Activate environment

# Work
pixi run build         # Build project
pixi run test          # Run tests
```

### For mise Users
```bash
# Setup
pixi install           # Install deps (includes mise!)
pixi shell             # Activate environment

# Work (using mise)
mise run build         # Build project
mise run test          # Run tests
mise tasks             # List available tasks
```

### For CI/CD
```yaml
# Use pixi in GitHub Actions (more reliable)
- uses: prefix-dev/setup-pixi@v0.9.3
- run: pixi run build
- run: pixi run test
```

## Integration Points

### 1. Environment Variables
Both tools set the same environment:
- `AGENTASKIT_ROOT` - Project root
- `CARGO_HOME` - Rust cargo directory
- `PATH` - Includes pixi/mise-managed tools

### 2. Tool Versions
Single source of truth: pixi.toml
```toml
[dependencies]
python = ">=3.11"      # mise uses this too
rust = ">=1.70"        # mise uses this too
nodejs = ">=20"        # mise uses this too
```

### 3. Task Execution
Tasks are mirrored between configs:
- pixi.toml tasks → Simple, direct commands
- .mise.toml tasks → Can add dependencies, env vars

## Migration Path (Future)

If we decide to consolidate (not now):
1. Keep pixi.toml as primary
2. Deprecate .mise.toml gradually
3. Provide migration guide

But for now: **Both coexist harmoniously** ✅

## Troubleshooting

### "mise: command not found"
```bash
# Make sure you're in pixi shell
pixi shell

# Or use pixi to run mise
pixi run mise --version
```

### "Tasks differ between pixi and mise"
Check both configs:
```bash
pixi task list
mise tasks
```

Update .mise.toml to match pixi.toml tasks.

### "Which should I use?"
**Recommendation for AgentAskit:**
- Use **pixi** for most tasks (simpler, better CI/CD)
- Use **mise** if you already know it or need advanced task features

Both work! Choose based on preference.

---

**Last Updated:** 2026-01-03
**Status:** ✅ Integrated - Both tools working together
