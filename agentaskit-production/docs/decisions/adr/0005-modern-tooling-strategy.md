# ADR-0005: Modern Development Tooling Strategy

**Status:** Accepted
**Date:** 2025-10-05
**Decision Makers:** @platform, @program

## Context

The AgentAskit project requires a unified, cross-platform development environment that supports:
- Consistent tool versioning across developer machines
- AI-assisted development workflows
- Cross-platform shell compatibility (Nushell)
- Centralized dotfile management
- XDG Base Directory compliance

## Decision

We adopt the following modern tooling stack:

### 1. Environment & Task Management

**mise + Pixi** - Unified tool/environment/task management

| Tool | Purpose | Integration |
|------|---------|-------------|
| mise | Runtime version management (Node, Python, Rust) | `.mise.toml` |
| Pixi | Conda-compatible package management | `pixi.toml` |

Configuration:
```toml
# .mise.toml
[tools]
node = "20"
python = "3.11"
rust = "stable"

[env]
PROJECT_ROOT = "{{cwd}}"

[tasks.build]
run = "cargo build --release"
```

### 2. Shell Integration

**direnv + Nushell** - Automatic environment loading

```nu
# ~/.config/nushell/config.nu
$env.config = {
    hooks: {
        pre_prompt: [{ ||
            if (which direnv | is-not-empty) {
                direnv export json | from json | default {} | load-env
            }
        }]
    }
}
```

### 3. AI Pair Programming

**aider** - Terminal-based AI coding assistant

```bash
# Installation
pip install aider-chat

# Usage with local models
aider --model ollama/codellama
```

Integration with NOA:
- Uses same model endpoints as NOA agents
- Shares context with orchestration system
- Logs sessions to `TEST/aider/`

### 4. Dotfile Management

**chezmoi** - Centralized dotfile manager integrated with NOA

```bash
# Initialize
chezmoi init

# Structure
~/.local/share/chezmoi/
├── dot_config/
│   ├── nushell/
│   ├── mise/
│   └── direnv/
├── .chezmoiignore
└── .chezmoi.toml.tmpl
```

Integration:
- NOA manages dotfile templates
- Chezmoi applies per-machine variations
- Secrets handled via age encryption

### 5. XDG Compliance

**xdg-ninja** - Audit and enforce XDG Base Directory spec

```bash
# Run audit
xdg-ninja

# Save report
xdg-ninja --json > tools/analysis/xdg_audit.json
```

Goals:
- Eliminate `$HOME` pollution
- Centralize configs under `~/.config/`
- Cache under `~/.cache/`
- Data under `~/.local/share/`

### 6. Zero Install Integration

**0install** via Pixi for distribution feeds:

```bash
pixi global install 0install

# Create feeds
0install publish ./core/feeds/agentaskit.xml
```

### 7. FlexNetOS Tool Ecosystem

Clone and integrate the following repositories:

| Repository | Purpose |
|------------|---------|
| FlexNetOS/pixi | Package management fork |
| FlexNetOS/nushell | Cross-platform shell |
| FlexNetOS/coreutils | Rust-based utilities |
| dan-t/rusty-tags | Rust ctags generator |
| FlexNetOS/syn | Rust parser for tooling |

## Consequences

### Positive

- Consistent development environments across all platforms
- AI-assisted development built into workflow
- Clean, XDG-compliant home directories
- Unified task running (mise tasks)
- Cross-platform compatibility via Nushell

### Negative

- Learning curve for new tooling
- Initial setup complexity
- Dependency on multiple tools

### Neutral

- Replaces existing ad-hoc tool management
- Requires documentation updates

## Implementation

1. Install mise and configure `.mise.toml`
2. Install Pixi and configure `pixi.toml`
3. Configure direnv with Nushell integration
4. Install aider and configure model endpoints
5. Initialize chezmoi with NOA templates
6. Run xdg-ninja audit and remediate
7. Clone FlexNetOS tool repositories
8. Document in onboarding guide

## Evidence

- Configuration: `.mise.toml`, `pixi.toml`
- Shell config: `~/.config/nushell/`
- Dotfiles: `~/.local/share/chezmoi/`
- ADR: This document

## Related

- [GOV-ADR](../../.todo) - ADR tracking
- [DOC-001](../../.todo) - Documentation
- [WF-001](../../.todo) - Development workflows
