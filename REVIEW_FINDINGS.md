# AgentAskit Infrastructure Review - Critical Findings

**Date:** 2026-01-03
**Reviewer:** Claude (Automated Review)
**Scope:** Shell configuration, package management, workflow systems

---

## Executive Summary

The project has **good foundations** but **critical gaps** in tooling consistency:

✅ **Strengths:**
- Nu shell configurations exist and are well-designed
- Pixi.toml is configured with basic dependencies
- Workflows reference Nu shell and pixi

❌ **Critical Issues:**
- **Duplicate tool management** (mise.toml + pixi.toml)
- **Missing pnpm** in pixi dependencies
- **Workflows use bash wrapper** instead of native Nu shell
- **No default shell configuration** - Nu shell not activated automatically
- **Missing pixi-managed toolchains** - cargo, python, pnpm not via pixi

---

## Detailed Findings

### 1. Shell Configuration Issues

#### ❌ Nu Shell NOT Default
**Location:** System-wide shell configuration
**Issue:** Nu shell exists but is not configured as the default shell

**Current State:**
- Nu shell config files exist in `configs/nushell/`
- Users must manually source: `source /path/to/agentaskit/configs/nushell/config.nu`
- No automatic activation mechanism

**Impact:** Developers use bash/zsh by default, Nu shell unused

**Fix Required:**
- Create shell activation script for all platforms
- Add to shell rc files (.bashrc, .zshrc, PowerShell profile)
- Provide pixi activation hook

---

### 2. Package Manager Duplication

#### ❌ Both mise.toml AND pixi.toml Exist
**Locations:**
- `.mise.toml` (101 lines)
- `pixi.toml` (79 lines)

**Issue:** Two competing package managers with overlapping configs

**Current State:**
- **.mise.toml** manages: rust, node, python, tasks
- **pixi.toml** manages: python, rust, nushell, tasks
- Duplication causes confusion and inconsistency

**Impact:**
- Unclear which tool to use
- Different tool versions possible
- Maintenance burden

**Fix Required:**
- **DECISION**: Use pixi as PRIMARY (per user requirement)
- Deprecate or remove .mise.toml
- Migrate all mise tasks to pixi

---

### 3. Missing Dependencies in pixi.toml

#### ❌ pnpm Not Included
**Location:** `pixi.toml` [dependencies] section
**User Requirement:** "pixi as the default package manager for all other packages including python, cargo, and pnpm"

**Current State:**
```toml
[dependencies]
python = ">=3.11"
rust = ">=1.70"
nushell = ">=0.90"
bash = "*"
# ❌ Missing: pnpm, nodejs
```

**Impact:** pnpm not available via pixi, must install separately

**Fix Required:**
- Add `nodejs = ">=20"` (required for pnpm)
- Add `pnpm = ">=8"`
- Add `cargo-binstall = "*"` (faster cargo installs)

---

### 4. Workflows Use Bash Wrapper Instead of Nu Shell

#### ❌ Inefficient Shell Usage in GitHub Actions
**Locations:** All `.github/workflows/*.yml` files

**Issue:** Workflows declare `shell: bash`, then run `pixi run nu -c '...'`

**Current Pattern:**
```yaml
- name: Build
  shell: bash  # ❌ Uses bash as wrapper
  run: |
    pixi run nu -c '
      print "Building..."
      cargo build
    '
```

**Impact:**
- Extra process overhead (bash → pixi → nu)
- Harder to debug
- Not using Nu shell natively
- Harder to read multi-line commands

**Fix Required:**
- Use `shell: nushell {0}` or custom Nu shell runner
- Direct Nu shell execution without bash wrapper
- OR: Create reusable Nu shell scripts in `tools/scripts/` and call them

**Recommended Pattern:**
```yaml
- name: Build
  shell: bash
  run: pixi run --locked build  # Use pixi tasks directly
```

---

### 5. No Pixi-Managed Cargo/Python/pnpm

#### ❌ Toolchains Not Integrated with Pixi
**User Requirement:** "nu shell must use pixi as the default package manager for all other packages including python, cargo, and pnpm"

**Current State:**
- Python: Managed by pixi ✅ (via conda-forge)
- Rust/Cargo: Managed by pixi ✅ (via conda-forge)
- pnpm: **NOT in pixi** ❌
- Node.js: **NOT in pixi** ❌ (required for pnpm)

**Issue:** When user runs `cargo`, `python`, or `pnpm`, they get:
- System version (if installed)
- OR: mise-managed version (if mise is active)
- NOT: pixi-managed version

**Impact:**
- Inconsistent toolchain versions across developers
- Pixi environment not fully isolated

**Fix Required:**
1. Add all tools to pixi.toml
2. Ensure `pixi shell` activates PATH with pixi-provided binaries first
3. Update env.nu to use pixi-provided tools
4. Document: "Always use `pixi run <cmd>` or `pixi shell`"

---

### 6. No Automatic Pixi Environment Activation

#### ❌ No Shell Integration for Pixi
**Location:** Shell configuration files

**Issue:** Pixi environment not automatically activated when entering project

**Current State:**
- Must manually run `pixi shell` or `pixi run <task>`
- Nu shell config.nu has commented-out directory hooks
- No automatic environment loading

**Impact:**
- Developers forget to activate pixi
- Use wrong toolchain versions
- Inconsistent development environment

**Fix Required:**
- Enable pixi shell auto-activation
- Add to Nu shell hooks in config.nu
- Provide activation scripts for bash/zsh/PowerShell users

---

## Priority Fix List

### 🔴 CRITICAL (Must Fix Immediately)

1. **Add pnpm + nodejs to pixi.toml**
2. **Create pixi shell activation mechanism**
3. **Consolidate mise.toml into pixi.toml** (or clearly document strategy)
4. **Fix workflows to use pixi tasks natively**

### 🟡 HIGH (Should Fix Soon)

5. **Make Nu shell default** (via shell rc files or documentation)
6. **Add pixi.lock to git** (for reproducible builds)
7. **Create tools/scripts/ with Nu shell scripts** (instead of inline YAML)

### 🟢 MEDIUM (Nice to Have)

8. **Add more pixi-managed tools** (cargo-binstall, cargo-watch, etc.)
9. **Create platform-specific pixi.toml features**
10. **Document tool management strategy in ADR**

---

## Recommended Architecture

### Unified Tool Management Strategy

```
┌─────────────────────────────────────────────┐
│           pixi (Primary Package Manager)     │
│  - Manages: python, rust, nushell, nodejs,  │
│             pnpm, all dev dependencies       │
└─────────────────────────────────────────────┘
                    │
        ┌───────────┴───────────┐
        ▼                       ▼
┌───────────────┐       ┌──────────────┐
│  Nu Shell     │       │  Workflows   │
│  (Default)    │       │  (CI/CD)     │
│  - env.nu     │       │  - Use pixi  │
│  - config.nu  │       │    tasks     │
└───────────────┘       └──────────────┘
        │                       │
        └───────────┬───────────┘
                    ▼
        ┌───────────────────────┐
        │  Developer Machine    │
        │  - pixi shell active  │
        │  - Nu shell default   │
        │  - All tools via pixi │
        └───────────────────────┘
```

---

## Next Steps

1. **Review with team** - Confirm pixi-only strategy
2. **Apply fixes** - Systematically implement changes
3. **Test across platforms** - Verify Linux, macOS, Windows
4. **Update documentation** - README, ADRs, CONTRIBUTING.md
5. **Announce to team** - Migration guide for developers

---

## Files Requiring Changes

### Configuration Files
- [ ] `pixi.toml` - Add pnpm, nodejs, activation scripts
- [ ] `configs/nushell/env.nu` - Add pixi activation
- [ ] `configs/nushell/config.nu` - Enable directory hooks
- [ ] `.mise.toml` - Deprecate or remove (decide)

### Workflow Files (21 files)
- [ ] `.github/workflows/ci.yml`
- [ ] `.github/workflows/rust.yml`
- [ ] `.github/workflows/verify.yml`
- [ ] `.github/workflows/sbom.yml`
- [ ] `.github/workflows/todo-validate.yml`
- [ ] (16 more workflow files...)

### Documentation
- [ ] `README.md` - Update setup instructions
- [ ] `docs/ADR-0005-modern-tooling.md` - Update strategy
- [ ] `CONTRIBUTING.md` - Document pixi usage

---

## Open Questions

1. **Should we keep mise.toml?**
   - Option A: Remove completely, use only pixi
   - Option B: Keep for tasks only, pixi for packages
   - **Recommendation:** Remove (pixi handles both)

2. **How to enforce pixi usage?**
   - Option A: CI fails if not using pixi
   - Option B: Documentation only
   - **Recommendation:** CI enforcement

3. **Should Nu shell be REQUIRED?**
   - Option A: Required (no bash support)
   - Option B: Optional (bash still works)
   - **Recommendation:** Required for scripts, optional for developer shell

---

**END OF REVIEW**
