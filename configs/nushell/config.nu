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

# AgentAskit project aliases
alias aa = cd $env.AGENTASKIT_ROOT
alias aab = cargo build --release
alias aat = cargo test --all
alias aal = cargo clippy --all-targets

# Custom commands for development
def "todo list" [] {
    # Parse and display .todo items
    open ($env.AGENTASKIT_ROOT | path join "agentaskit-production" ".todo")
    | lines
    | where ($it | str starts-with "- [")
    | each { |line|
        let status = if ($line | str contains "[x]") { "✓" } else { "○" }
        let ref = ($line | parse --regex '\[REF: ([^\]]+)\]' | get -i 0.capture0 | default "")
        {status: $status, ref: $ref, line: $line}
    }
}

def "build tools" [] {
    # Build external tools from source
    print "Building pixi..."
    cd ($env.AGENTASKIT_ROOT | path join "tools" "external" "pixi")
    cargo build --release

    print "Building nushell..."
    cd ($env.AGENTASKIT_ROOT | path join "tools" "external" "nushell")
    cargo build --release

    print "Building coreutils..."
    cd ($env.AGENTASKIT_ROOT | path join "tools" "external" "coreutils")
    cargo build --release

    print "Building rusty-tags..."
    cd ($env.AGENTASKIT_ROOT | path join "tools" "external" "rusty-tags")
    cargo build --release
}

# Starship prompt (optional)
# $env.STARSHIP_CONFIG = ($env.XDG_CONFIG_HOME | path join "starship.toml")
