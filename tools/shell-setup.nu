# AgentAskit Shell Setup Helper
# This script helps developers configure their shell to use Nu shell with pixi
# REF: ADR-0005 Modern Tooling Strategy

print "(ansi green_bold)═══════════════════════════════════════════════════════════════(ansi reset)"
print "(ansi green_bold)     AgentAskit Shell Setup - Nu Shell + Pixi Integration(ansi reset)"
print "(ansi green_bold)═══════════════════════════════════════════════════════════════(ansi reset)"
print ""

let project_root = (pwd)
let config_dir = ($project_root | path join "configs" "nushell")

# Detect current shell
let current_shell = if ($nu.os-info.name == "windows") {
    "powershell"
} else {
    $env.SHELL? | default "unknown" | path basename
}

print $"Current detected shell: (ansi cyan)($current_shell)(ansi reset)"
print ""

# Function to display setup instructions
def show-instructions [shell: string] {
    print $"(ansi yellow_bold)Setup Instructions for ($shell):(ansi reset)"
    print ""

    if ($shell == "bash") {
        print "Add this to your ~/.bashrc:"
        print "```bash"
        print $"# AgentAskit - Nu shell integration"
        print $"export AGENTASKIT_ROOT=\"($project_root)\""
        print $"alias agentaskit-shell='pixi shell --manifest-path ($project_root)/pixi.toml'"
        print $"alias aa='cd ($project_root) && pixi shell'"
        print "```"
    } else if ($shell == "zsh") {
        print "Add this to your ~/.zshrc:"
        print "```zsh"
        print $"# AgentAskit - Nu shell integration"
        print $"export AGENTASKIT_ROOT=\"($project_root)\""
        print $"alias agentaskit-shell='pixi shell --manifest-path ($project_root)/pixi.toml'"
        print $"alias aa='cd ($project_root) && pixi shell'"
        print "```"
    } else if ($shell == "fish") {
        print "Add this to your ~/.config/fish/config.fish:"
        print "```fish"
        print $"# AgentAskit - Nu shell integration"
        print $"set -gx AGENTASKIT_ROOT \"($project_root)\""
        print $"alias agentaskit-shell='pixi shell --manifest-path ($project_root)/pixi.toml'"
        print $"alias aa='cd ($project_root); and pixi shell'"
        print "```"
    } else if ($shell == "powershell" or $shell == "pwsh") {
        let profile_path = if ($nu.os-info.name == "windows") {
            "$env:USERPROFILE\\Documents\\PowerShell\\Microsoft.PowerShell_profile.ps1"
        } else {
            "~/.config/powershell/Microsoft.PowerShell_profile.ps1"
        }
        print $"Add this to your PowerShell profile: (ansi cyan)($profile_path)(ansi reset)"
        print "```powershell"
        print $"# AgentAskit - Nu shell integration"
        print $"$env:AGENTASKIT_ROOT = \"($project_root)\""
        print $"function agentaskit-shell {{ pixi shell --manifest-path ($project_root)/pixi.toml }}"
        print $"function aa {{ cd ($project_root); pixi shell }}"
        print "```"
    } else if ($shell == "nu" or $shell == "nushell") {
        let nu_config = if ($nu.os-info.name == "windows") {
            "$env:APPDATA\\nushell\\config.nu"
        } else {
            "~/.config/nushell/config.nu"
        }
        print $"Add this to your Nu config: (ansi cyan)($nu_config)(ansi reset)"
        print "```nushell"
        print $"# AgentAskit integration"
        print $"source ($config_dir)/config.nu"
        print "```"
    }

    print ""
    print "(ansi green)Quick start after setup:(ansi reset)"
    print "  1. Restart your shell or source the config file"
    print "  2. Navigate to the AgentAskit directory"
    print $"  3. Run: (ansi cyan_bold)pixi shell(ansi reset) to activate the environment"
    print $"  4. Run: (ansi cyan_bold)env info(ansi reset) to verify the setup"
    print ""
}

# Main menu
print "Choose your shell to see setup instructions:"
print "  1. Bash"
print "  2. Zsh"
print "  3. Fish"
print "  4. PowerShell/pwsh"
print "  5. Nushell (you're already here!)"
print "  6. Show all"
print ""

let choice = (input "(ansi green)Enter choice (1-6):(ansi reset) ")

match $choice {
    "1" => { show-instructions "bash" }
    "2" => { show-instructions "zsh" }
    "3" => { show-instructions "fish" }
    "4" => { show-instructions "powershell" }
    "5" => { show-instructions "nu" }
    "6" => {
        show-instructions "bash"
        print ""
        print "───────────────────────────────────────────────────────────────"
        print ""
        show-instructions "zsh"
        print ""
        print "───────────────────────────────────────────────────────────────"
        print ""
        show-instructions "fish"
        print ""
        print "───────────────────────────────────────────────────────────────"
        print ""
        show-instructions "powershell"
        print ""
        print "───────────────────────────────────────────────────────────────"
        print ""
        show-instructions "nu"
    }
    _ => {
        print $"(ansi red)Invalid choice. Please run again and choose 1-6.(ansi reset)"
    }
}

print ""
print "(ansi cyan_bold)═══════════════════════════════════════════════════════════════(ansi reset)"
print "  For more information, see: docs/ADR-0005-modern-tooling.md"
print "(ansi cyan_bold)═══════════════════════════════════════════════════════════════(ansi reset)"
