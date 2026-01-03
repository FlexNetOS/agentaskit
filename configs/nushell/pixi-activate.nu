# Pixi Activation Script for Nushell
# This script is automatically run when entering a pixi shell
# REF: ADR-0005 Modern Tooling Strategy

print "🔧 Activating AgentAskit pixi environment..."

# Verify we're in a pixi shell
if ($env.PIXI_IN_SHELL? | is-empty) {
    $env.PIXI_IN_SHELL = "true"
}

# Set project root
let project_root_env = ($env.PIXI_PROJECT_ROOT? | default "")
let project_root = if ($project_root_env | is-empty) {
    # Fall back to current working directory, but warn if it doesn't look like the project root
    let cwd = (pwd)
    let expected_env_file = ($cwd | path join "configs" "nushell" "env.nu")
    if (not ($expected_env_file | path exists)) {
        print "⚠️  Warning: PIXI_PROJECT_ROOT is not set and current directory does not look like the AgentAskit project root (missing configs/nushell/env.nu). Paths may be incorrect."
    }
    $cwd
} else {
    $project_root_env
}
$env.AGENTASKIT_ROOT = $project_root

# Source the full Nu shell environment if not already loaded
let env_file = ($project_root | path join "configs" "nushell" "env.nu")
if ($env_file | path exists) {
    # Only source if AGENTASKIT_ENV_LOADED is not set (avoid double-loading)
    if ($env.AGENTASKIT_ENV_LOADED? | is-empty) {
        source $env_file
        $env.AGENTASKIT_ENV_LOADED = "true"
    }
}

# Verify pixi-managed tools are available
let tools = ["python", "cargo", "nu", "pnpm", "node"]
for tool in $tools {
    if (which $tool | is-empty) {
        print $"⚠️  Warning: ($tool) not found in PATH"
    }
}

print $"✅ Pixi environment activated for (ansi green_bold)AgentAskit(ansi reset)"
print $"   Project root: ($env.AGENTASKIT_ROOT)"
print $"   Shell: Nushell (char lparen)(version | get version)(char rparen)"
print ""
print "Available pixi tasks:"
print "  pixi run build       - Build the project"
print "  pixi run test        - Run tests"
print "  pixi run lint        - Run linters"
print "  pixi run bootstrap   - Bootstrap dev environment"
print ""
print "Type 'env info' for environment details"
