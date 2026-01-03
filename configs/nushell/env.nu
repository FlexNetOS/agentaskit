# AgentAskit Nushell Environment Configuration
# REF: ADR-0005 Modern Tooling Strategy
# This is the PRIMARY environment configuration - cross-platform

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
        | where { |line| $line | str starts-with "- [" }
        | each { |line|
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
    print $"AGENTASKIT_ROOT: ($env.AGENTASKIT_ROOT)"
    print $"XDG_CONFIG_HOME: ($env.XDG_CONFIG_HOME)"
    print $"XDG_DATA_HOME:   ($env.XDG_DATA_HOME)"
    print $"CARGO_HOME:      ($env.CARGO_HOME)"
    print $"Tools bin:       ($env.AGENTASKIT_ROOT)/tools/bin"
}
