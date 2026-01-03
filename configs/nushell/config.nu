# Nushell Configuration for AgentAskit
# REF: ADR-0005 Modern Tooling Strategy
# This file should be sourced from ~/.config/nushell/config.nu

# XDG Base Directory paths
$env.XDG_CONFIG_HOME = ($env.HOME | path join ".config")
$env.XDG_DATA_HOME = ($env.HOME | path join ".local" "share")
$env.XDG_CACHE_HOME = ($env.HOME | path join ".cache")
$env.XDG_STATE_HOME = ($env.HOME | path join ".local" "state")

# Rust XDG compliance
$env.CARGO_HOME = ($env.XDG_DATA_HOME | path join "cargo")
$env.RUSTUP_HOME = ($env.XDG_DATA_HOME | path join "rustup")

# Direnv integration helper
def --env "direnv-load" [] {
    # Ensure direnv is available
    if (which direnv | is-not-empty) {
        let current_pwd = $env.PWD

        # Avoid re-running direnv for the same directory
        if ('DIRENV_LAST_PWD' in $env and $env.DIRENV_LAST_PWD == $current_pwd) {
            return
        }

        # Export direnv environment
        let direnv_export = (direnv export json | complete)
        if $direnv_export.exit_code == 0 and ($direnv_export.stdout | str length) > 0 {
            $direnv_export.stdout | from json | default {} | load-env
            $env.DIRENV_LAST_PWD = $current_pwd
        }
    }
}

# Direnv integration hook
# This runs before each prompt to load .envrc files
$env.config = ($env.config | merge {
    hooks: {
        pre_prompt: [{ ||
            # Load direnv environment for the current directory (if needed)
            direnv-load
        }]
        env_change: {
            PWD: [{ |before, after|
                # Trigger direnv on directory change
                direnv-load
            }]
        }
    }
})

# Mise integration
def --env "mise-activate" [] {
    if (which mise | is-not-empty) {
        mise activate nu | save -f ($env.XDG_CACHE_HOME | path join "mise.nu")
        source ($env.XDG_CACHE_HOME | path join "mise.nu")
    }
}

# AgentAskit project aliases (require AGENTASKIT_ROOT to be set)
# Use 'mise activate' or source .envrc to set AGENTASKIT_ROOT
def "aa" [] {
    let root = ($env | get -i AGENTASKIT_ROOT | default "")
    if $root == "" {
        print "error: AGENTASKIT_ROOT is not set. Run 'mise activate' or source .envrc first."
    } else {
        cd $root
    }
}
alias aab = cargo build --release
alias aat = cargo test --all
alias aal = cargo clippy --all-targets

# Custom commands for development
def "todo list" [] {
    # Ensure AGENTASKIT_ROOT is set before attempting to read the .todo file
    let root = ($env | get -i AGENTASKIT_ROOT | default "")
    if $root == "" {
        print "error: AGENTASKIT_ROOT is not set. Make sure you're in the AgentAskit project context and that mise (or your env) has been activated."
    } else {
        # Parse and display .todo items
        open ($root | path join "agentaskit-production" ".todo")
        | lines
        | where ($it | str starts-with "- [")
        | each { |line|
            let status = if ($line | str contains "[x]") { "✓" } else { "○" }
            let ref = ($line | parse --regex '\[REF: ([^\]]+)\]' | get -i 0.capture0 | default "")
            {status: $status, ref: $ref, line: $line}
        }
    }
}

def "build tools" [] {
    # Build external tools from source without changing the working directory
    print "Building pixi..."
    cargo build --manifest-path ($env.AGENTASKIT_ROOT | path join "tools" "external" "pixi" "Cargo.toml") --release

    print "Building nushell..."
    cargo build --manifest-path ($env.AGENTASKIT_ROOT | path join "tools" "external" "nushell" "Cargo.toml") --release

    print "Building coreutils..."
    cargo build --manifest-path ($env.AGENTASKIT_ROOT | path join "tools" "external" "coreutils" "Cargo.toml") --release

    print "Building rusty-tags..."
    cargo build --manifest-path ($env.AGENTASKIT_ROOT | path join "tools" "external" "rusty-tags" "Cargo.toml") --release
}

# Starship prompt (optional)
# $env.STARSHIP_CONFIG = ($env.XDG_CONFIG_HOME | path join "starship.toml")
