# External Tools

This directory contains cloned external tool repositories for AgentAsKit development.

## Repositories

Clone these repositories to set up the development environment:

```bash
# FlexNetOS tools
git clone https://github.com/FlexNetOS/pixi.git
git clone https://github.com/FlexNetOS/nushell.git
git clone https://github.com/FlexNetOS/coreutils.git
git clone https://github.com/FlexNetOS/syn.git

# Third-party tools
git clone https://github.com/dan-t/rusty-tags.git
```

## Building

Build all tools with:

```bash
# Using mise task
mise run build-tools

# Or manually
for dir in pixi nushell coreutils rusty-tags; do
    (cd "$dir" && cargo build --release)
done
```

## Purpose

| Tool | Purpose |
|------|---------|
| pixi | Package management with 0install support |
| nushell | Cross-platform shell |
| coreutils | Rust-based Unix utilities |
| rusty-tags | Rust ctags generator |
| syn | Rust parser for tooling |

See [ADR-0005](../../agentaskit-production/docs/decisions/adr/0005-modern-tooling-strategy.md) for details.
