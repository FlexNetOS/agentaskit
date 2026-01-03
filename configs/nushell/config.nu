# Nushell Configuration for AgentAskit
# REF: ADR-0005 Modern Tooling Strategy
#
# Usage: Add to your ~/.config/nushell/config.nu:
#   source /path/to/agentaskit/configs/nushell/config.nu
#
# Or for env-only (no hooks):
#   source /path/to/agentaskit/configs/nushell/env.nu

# Source the environment configuration
source ($env.FILE_PWD | path join "env.nu")

# Mise integration (optional - activates mise for this project)
def --env "mise-activate" [] {
    if (which mise | is-not-empty) {
        let cache_file = ($env.XDG_CACHE_HOME | path join "mise.nu")
        mise activate nu | save -f $cache_file
        source $cache_file
    }
}

# Starship prompt (optional - uncomment if using starship)
# $env.STARSHIP_CONFIG = ($env.XDG_CONFIG_HOME | path join "starship.toml")

# Directory change hook - auto-load project environment
$env.config = ($env.config | merge {
    hooks: {
        env_change: {
            PWD: [{ |before, after|
                # Check if entering an AgentAskit project
                let cargo_toml = ($after | path join "Cargo.toml")
                if ($cargo_toml | path exists) {
                    # Could auto-source env.nu here if desired
                    # source ($after | path join "configs" "nushell" "env.nu")
                }
            }]
        }
    }
})
