# Chezmoi Dotfile Management

AgentAsKit includes [chezmoi](https://chezmoi.io/) for cross-platform dotfile management.

## Quick Start

```bash
# Initialize chezmoi with this config
chezmoi init

# Copy the config template
cp configs/chezmoi/chezmoi.toml ~/.config/chezmoi/chezmoi.toml

# Add your first dotfile
chezmoi add ~/.bashrc

# See what would change
chezmoi diff

# Apply changes
chezmoi apply
```

## Nushell Integration

Add AgentAsKit's nushell config to your dotfiles:

```bash
# Add the config directory
chezmoi add ~/.config/nushell/

# Or create a symlink to always use project config
chezmoi add --template ~/.config/nushell/config.nu
```

## Template Variables

In `chezmoi.toml`, set your personal data:

```toml
[data]
    project = "agentaskit"
    email = "you@example.com"
    name = "Your Name"
```

Use in templates:

```
# .bashrc.tmpl
export GIT_AUTHOR_EMAIL="{{ .email }}"
```

## Cross-Platform Templates

```bash
# .bashrc.tmpl
{{ if eq .chezmoi.os "darwin" -}}
export HOMEBREW_PREFIX="/opt/homebrew"
{{ else if eq .chezmoi.os "linux" -}}
export HOMEBREW_PREFIX="/home/linuxbrew/.linuxbrew"
{{ end -}}
```

## With Nushell Scripts

Add `.chezmoiscripts/` with nushell:

```nu
#!/usr/bin/env nu
# run_once_install-packages.nu

print "Installing packages..."
```

## Learn More

- [Chezmoi Quick Start](https://chezmoi.io/quick-start/)
- [Template Guide](https://chezmoi.io/user-guide/templating/)
- [ADR-0005](../../agentaskit-production/docs/decisions/adr/0005-modern-tooling-strategy.md)
