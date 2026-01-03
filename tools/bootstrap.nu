#!/usr/bin/env nu
# AgentAsKit Development Environment Bootstrap
# REF: ADR-0005 Modern Tooling Strategy
# Cross-platform: Linux, macOS, Windows - single script

# Colors
def log-info [msg: string] { print $"(ansi cyan)[INFO](ansi reset) ($msg)" }
def log-ok [msg: string] { print $"(ansi green)[OK](ansi reset) ($msg)" }
def log-warn [msg: string] { print $"(ansi yellow)[WARN](ansi reset) ($msg)" }
def log-err [msg: string] { print $"(ansi red)[ERROR](ansi reset) ($msg)" }

# Detect OS
def detect-os [] {
    let os = (sys host | get name)
    let arch = (sys host | get arch)
    log-info $"Detected: ($os) \(($arch)\)"
    { os: $os, arch: $arch }
}

# Get project root (where this script lives)
def get-project-root [] {
    $env.FILE_PWD | path dirname
}

# Check prerequisites
def check-prereqs [] {
    log-info "Checking prerequisites..."

    let missing = (
        [
            (if (which git | is-empty) { "git" } else { null })
            (if (which cargo | is-empty) { "rust/cargo" } else { null })
        ]
        | where {|it| $it != null }
    )
    if ($missing | is-not-empty) {
        log-err $"Missing prerequisites: ($missing | str join ', ')"
        exit 1
    }

    log-ok "All prerequisites found"
}

# Clone external repositories
def clone-repos [tools_dir: string] {
    log-info "Cloning external tool repositories..."

    mkdir $tools_dir
    cd $tools_dir

    let repos = [
        "https://github.com/FlexNetOS/pixi.git"
        "https://github.com/FlexNetOS/nushell.git"
        "https://github.com/FlexNetOS/coreutils.git"
        "https://github.com/FlexNetOS/syn.git"
        "https://github.com/dan-t/rusty-tags.git"
        "https://github.com/twpayne/chezmoi.git"
    ]

    for repo in $repos {
        let name = ($repo | path basename | str replace ".git" "")
        if ($name | path exists) {
            log-info $"  ($name) already cloned"
        } else {
            log-info $"  Cloning ($name)..."
            try {
                git clone --depth 1 $repo
            } catch {
                log-warn $"  Failed to clone ($name)"
            }
        }
    }

    log-ok "Repositories ready"
}

# Build tools from source
def build-tools [tools_dir: string] {
    log-info "Building tools from source..."

    let rust_tools = ["nushell" "coreutils" "rusty-tags"]

    for tool in $rust_tools {
        let tool_path = ($tools_dir | path join $tool)
        if ($tool_path | path exists) {
            log-info $"  Building ($tool)..."
            cd $tool_path
            try {
                cargo build --release
                log-ok $"  ($tool) built"
            } catch {
                log-warn $"  ($tool) build failed"
            }
        }
    }

    # Chezmoi (Go)
    let chezmoi_path = ($tools_dir | path join "chezmoi")
    if ($chezmoi_path | path exists) and (which go | is-not-empty) {
        log-info "  Building chezmoi..."
        cd $chezmoi_path
        try {
            git checkout v2.52.0
            $env.GOPROXY = "direct"
            go build -o chezmoi .
            log-ok "  chezmoi built"
        } catch {
            log-warn "  chezmoi build failed"
        }
    }
}

# Create bin directory with symlinks
def create-bin-links [project_root: string, tools_dir: string] {
    log-info "Creating portable bin directory..."

    let bin_dir = ($project_root | path join "tools" "bin")
    mkdir $bin_dir

    # Nushell
    let nu_src = ($tools_dir | path join "nushell" "target" "release" "nu")
    let nu_dst = ($bin_dir | path join "nu")
    if ($nu_src | path exists) {
        rm -f $nu_dst
        # Use cp for Windows compatibility (symlinks need admin)
        if (sys host | get name) == "Windows" {
            cp $nu_src $nu_dst
        } else {
            ln -sf $nu_src $nu_dst
        }
        log-ok "  nu"
    }

    # Chezmoi
    let chezmoi_src = ($tools_dir | path join "chezmoi" "chezmoi")
    let chezmoi_dst = ($bin_dir | path join "chezmoi")
    if ($chezmoi_src | path exists) {
        rm -f $chezmoi_dst
        if (sys host | get name) == "Windows" {
            cp $chezmoi_src $chezmoi_dst
        } else {
            ln -sf $chezmoi_src $chezmoi_dst
        }
        log-ok "  chezmoi"
    }

    # Rusty-tags
    let rt_src = ($tools_dir | path join "rusty-tags" "target" "release" "rusty-tags")
    let rt_dst = ($bin_dir | path join "rusty-tags")
    if ($rt_src | path exists) {
        rm -f $rt_dst
        if (sys host | get name) == "Windows" {
            cp $rt_src $rt_dst
        } else {
            ln -sf $rt_src $rt_dst
        }
        log-ok "  rusty-tags"
    }

    log-info $"Add to PATH: ($bin_dir)"
}

# Install Python tools
def install-python-tools [] {
    log-info "Installing Python tools..."

    if (which aider | is-not-empty) {
        log-ok "aider already installed"
    } else {
        log-info "  Installing aider..."
        try {
            pip3 install --user --upgrade aider-chat
            log-ok "  aider installed"
        } catch {
            log-warn "  aider install failed"
        }
    }
}

# Configure aider
def configure-aider [project_root: string] {
    log-info "Configuring aider..."

    let config = '# Aider configuration for AgentAsKit
auto-commits: true
dirty-commits: false
attribute-author: true
gitignore: true
aiderignore: .aiderignore
dark-mode: true
pretty: true
stream: true
'

    $config | save -f ($project_root | path join ".aider.conf.yml")

    let ignore = '# Aider ignore patterns
tools/external/
target/
*.lock
.git/
node_modules/
__pycache__/
.env
.env.local
'

    $ignore | save -f ($project_root | path join ".aiderignore")

    log-ok "aider configured"
}

# Setup nushell config
def setup-nushell-config [project_root: string] {
    log-info "Setting up nushell configuration..."

    let xdg_config = ($env.XDG_CONFIG_HOME? | default ($env.HOME | path join ".config"))
    let nu_config_dir = ($xdg_config | path join "nushell")

    mkdir $nu_config_dir

    let loader = $'# AgentAsKit nushell configuration loader
# Source this from your main config.nu

$env.AGENTASKIT_ROOT = "($project_root)"

# Add project tools to PATH
$env.PATH = ($env.PATH | prepend "($project_root)/tools/bin")

# Project aliases
alias aa = cd $env.AGENTASKIT_ROOT
alias aab = cargo build --release
alias aat = cargo test --all
'

    $loader | save -f ($nu_config_dir | path join "agentaskit.nu")

    log-ok $"Nushell config created at ($nu_config_dir)/agentaskit.nu"
    log-info "Add to your config.nu: source ~/.config/nushell/agentaskit.nu"
}

# Print summary
def print-summary [project_root: string] {
    print ""
    print "============================================"
    print "  AgentAsKit Development Environment Setup"
    print "============================================"
    print ""
    log-info $"Project root: ($project_root)"
    log-info $"Tools bin:    ($project_root)/tools/bin"
    print ""
    print "Next steps:"
    print "  1. Add to your nushell config.nu:"
    print "     source ~/.config/nushell/agentaskit.nu"
    print ""
    print "  2. Or set PATH manually:"
    print $"     $env.PATH = ($env.PATH | prepend '($project_root)/tools/bin')"
    print ""
    print "  3. Configure aider API key:"
    print "     $env.ANTHROPIC_API_KEY = 'sk-ant-...'"
    print ""
}

# Main entry point
def main [
    --skip-build    # Skip building tools from source
    --skip-clone    # Skip cloning repositories
] {
    print "AgentAsKit Bootstrap Script (Nushell)"
    print "====================================="
    print ""

    let info = (detect-os)
    check-prereqs

    let project_root = (get-project-root)
    let tools_dir = ($project_root | path join "tools" "external")

    if not $skip_clone {
        clone-repos $tools_dir
    }

    if not $skip_build {
        build-tools $tools_dir
    }

    create-bin-links $project_root $tools_dir
    install-python-tools
    configure-aider $project_root
    setup-nushell-config $project_root

    print-summary $project_root

    log-ok "Bootstrap complete!"
}
