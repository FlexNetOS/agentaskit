# Pixi Activation Script for Nushell
# This script is automatically run when entering a pixi shell
# REF: ADR-0005 Modern Tooling Strategy

print "🔧 Activating AgentAskit pixi environment..."

# Verify we're in a pixi shell
if ($env.PIXI_IN_SHELL? | is-empty) {
    $env.PIXI_IN_SHELL = "true"
}

# Set project root
let project_root = ($env.PIXI_PROJECT_ROOT? | default (pwd))
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
