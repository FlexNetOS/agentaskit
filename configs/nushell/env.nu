# AgentAskit Nushell Environment Configuration
# REF: ADR-0005 Modern Tooling Strategy
# This is the PRIMARY environment configuration - cross-platform

# ==============================================================================
# Pixi Integration - Preferred Package Manager
# ==============================================================================

# Check if we're in a pixi shell and warn if not
if ($env.PIXI_IN_SHELL? | is-empty) {
    print "(ansi yellow_bold)⚠️  Not in pixi shell!(ansi reset)"
    print "   For best compatibility, activate pixi environment:"
    print "   (ansi green)pixi shell(ansi reset) or (ansi green)pixi run <task>(ansi reset)"
    print ""
}

# Ensure pixi-managed tools are prioritized in PATH
# Pixi sets PIXI_PROJECT_ROOT when active
if (not ($env.PIXI_PROJECT_ROOT? | is-empty)) {
    # Pixi is active - tools are already in PATH from pixi
    # Just record that we're using pixi
    $env.AGENTASKIT_PACKAGE_MANAGER = "pixi"
} else {
    # Pixi not active - try to detect pixi installation
    if not (which pixi | is-empty) {
        print "(ansi cyan)💡 Tip: Run 'pixi shell' to activate the pixi environment(ansi reset)"
    }
}

# Get project root (call from project directory)
def get-project-root [] {
    # Walk up to find Cargo.toml
    mut dir = (pwd)
    while ($dir != "/") and ($dir != "") {
        if ($dir | path join "Cargo.toml" | path exists) {
            return $dir
        }
        $dir = ($dir | path dirname)
    }
    pwd
}

# Set project root
$env.AGENTASKIT_ROOT = (get-project-root)

# XDG Base Directory compliance (works on all platforms)
$env.XDG_CONFIG_HOME = ($env.XDG_CONFIG_HOME? | default ($env.HOME | path join ".config"))
$env.XDG_DATA_HOME = ($env.XDG_DATA_HOME? | default ($env.HOME | path join ".local" "share"))
$env.XDG_CACHE_HOME = ($env.XDG_CACHE_HOME? | default ($env.HOME | path join ".cache"))
$env.XDG_STATE_HOME = ($env.XDG_STATE_HOME? | default ($env.HOME | path join ".local" "state"))

# Rust XDG compliance
$env.CARGO_HOME = ($env.CARGO_HOME? | default ($env.XDG_DATA_HOME | path join "cargo"))
$env.RUSTUP_HOME = ($env.RUSTUP_HOME? | default ($env.XDG_DATA_HOME | path join "rustup"))

# Add project tools to PATH
$env.PATH = ($env.PATH | prepend ($env.AGENTASKIT_ROOT | path join "tools" "bin"))
$env.PATH = ($env.PATH | prepend ($env.HOME | path join ".local" "bin"))

# Aider configuration
$env.AIDER_CONFIG = ($env.AGENTASKIT_ROOT | path join ".aider.conf.yml")

# Project aliases
alias aa = cd $env.AGENTASKIT_ROOT
alias aab = cargo build --release
alias aat = cargo test --all
alias aal = cargo clippy --all-targets

# Todo list helper
def "todo list" [] {
    let todo_file = ($env.AGENTASKIT_ROOT | path join "agentaskit-production" ".todo")
    if ($todo_file | path exists) {
        open $todo_file
        | lines
        | where {|line| $line | str starts-with "- [" }
        | each {|line|
            let status = if ($line | str contains "[x]") { "✓" } else { "○" }
            let ref = ($line | parse --regex '\[REF: ([^\]]+)\]' | get -i 0.capture0? | default "")
            { status: $status, ref: $ref, task: $line }
        }
    }
}

# Build tools helper
def "build tools" [] {
    let tools = ["nushell" "coreutils" "rusty-tags"]
    for tool in $tools {
        let path = ($env.AGENTASKIT_ROOT | path join "tools" "external" $tool)
        if ($path | path exists) {
            print $"Building ($tool)..."
            cd $path
            cargo build --release
        }
    }
}

# Print environment info
def "env info" [] {
    print "=== AgentAskit Environment ==="
    print $"AGENTASKIT_ROOT:     ($env.AGENTASKIT_ROOT)"
    print $"Package Manager:     ($env.AGENTASKIT_PACKAGE_MANAGER? | default 'not set')"
    print $"Pixi Active:         (if ($env.PIXI_IN_SHELL? | is-empty) { 'No ❌' } else { 'Yes ✅' })"
    print ""
    print "=== XDG Directories ==="
    print $"XDG_CONFIG_HOME:     ($env.XDG_CONFIG_HOME)"
    print $"XDG_DATA_HOME:       ($env.XDG_DATA_HOME)"
    print $"XDG_CACHE_HOME:      ($env.XDG_CACHE_HOME)"
    print $"CARGO_HOME:          ($env.CARGO_HOME)"
    print $"RUSTUP_HOME:         ($env.RUSTUP_HOME)"
    print ""
    print "=== Tool Versions ==="
    print $"Nu Shell:            (version | get version)"
    if not (which python | is-empty) { print $"Python:              (^python --version | str trim)" }
    if not (which cargo | is-empty) { print $"Cargo:               (^cargo --version | str trim)" }
    if not (which node | is-empty) { print $"Node.js:             (^node --version | str trim)" }
    if not (which pnpm | is-empty) { print $"pnpm:                (^pnpm --version | str trim)" }
}

# ==============================================================================
# AI Integration (aichat + claude-flow)
# ==============================================================================

# aichat configuration
$env.AICHAT_CONFIG_DIR = ($env.AGENTASKIT_ROOT | path join "configs" "aichat")

# AI completion using aichat CLI
def ai [prompt: string, --model (-m): string = ""] {
    let aichat = ($env.AGENTASKIT_ROOT | path join "integrations" "aichat" "target" "release" "aichat")
    if not ($aichat | path exists) {
        # Try system aichat
        if (which aichat | is-empty) {
            print "aichat not found. Build with: cd integrations/aichat && cargo build --release"
            return
        }
        if ($model | is-empty) {
            ^aichat $prompt
        } else {
            ^aichat -m $model $prompt
        }
    } else {
        if ($model | is-empty) {
            ^$aichat $prompt
        } else {
            ^$aichat -m $model $prompt
        }
    }
}

# List available AI providers/models
def "ai providers" [] {
    let aichat = ($env.AGENTASKIT_ROOT | path join "integrations" "aichat" "target" "release" "aichat")
    if ($aichat | path exists) {
        ^$aichat --list-models
    } else if not (which aichat | is-empty) {
        ^aichat --list-models
    } else {
        print "aichat not found"
    }
}

# claude-flow orchestration
def orchestrate [task: string, --agent (-a): string = "default"] {
    let cf = ($env.AGENTASKIT_ROOT | path join "integrations" "claude-flow" "bin" "claude-flow.js")
    if not ($cf | path exists) {
        print "claude-flow not found at integrations/claude-flow"
        return
    }
    ^node $cf run --agent $agent $task
}

# Start claude-flow swarm
def "swarm start" [name: string = "default"] {
    let cf = ($env.AGENTASKIT_ROOT | path join "integrations" "claude-flow" "bin" "claude-flow-swarm")
    if not ($cf | path exists) {
        print "claude-flow swarm not found"
        return
    }
    ^$cf start $name
}

# Agent gateway control
def "gateway start" [--config (-c): string = "integrated"] {
    let config_path = ($env.AGENTASKIT_ROOT | path join "configs" "agentgateway" $"($config).yaml")
    if not ($config_path | path exists) {
        print $"Config not found: ($config_path)"
        return
    }
    print $"Starting gateway with config: ($config_path)"
    # Gateway would be started here - placeholder for actual implementation
}

# Local inference helper
def "llama run" [prompt: string, --model (-m): string = "default.gguf"] {
    let llama = ($env.AGENTASKIT_ROOT | path join "integrations" "llama.cpp" "main")
    let model_path = ($env.AGENTASKIT_ROOT | path join "models" $model)
    if not ($llama | path exists) {
        print "llama.cpp not built. Build with: cd integrations/llama.cpp && make"
        return
    }
    if not ($model_path | path exists) {
        print $"Model not found: ($model_path)"
        return
    }
    ^$llama -m $model_path -p $prompt
}

# aichat nushell keybinding integration (Alt+E for inline AI)
def _aichat_nushell [] {
    let _prev = (commandline)
    if ($_prev != "") {
        print '⌛'
        commandline edit -r (ai $_prev)
    }
}

# Add keybinding for aichat integration (uncomment to enable)
# $env.config.keybindings = ($env.config.keybindings | append {
#     name: aichat_integration
#     modifier: alt
#     keycode: char_e
#     mode: [emacs, vi_insert]
#     event: [{ send: executehostcommand, cmd: "_aichat_nushell" }]
# })
