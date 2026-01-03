# AgentAskit Development Environment Bootstrap (Windows)
# REF: ADR-0005 Modern Tooling Strategy
# PowerShell 7+ recommended

#Requires -Version 5.1

param(
    [switch]$SkipBuild,
    [switch]$SkipClone,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$ToolsDir = Join-Path $ScriptDir "external"
$BinDir = Join-Path $ScriptDir "bin"

# Colors
function Write-Info { Write-Host "[INFO] $args" -ForegroundColor Cyan }
function Write-Success { Write-Host "[OK] $args" -ForegroundColor Green }
function Write-Warn { Write-Host "[WARN] $args" -ForegroundColor Yellow }
function Write-Err { Write-Host "[ERROR] $args" -ForegroundColor Red }

function Show-Help {
    Write-Host @"
AgentAskit Bootstrap Script (Windows)

Usage: .\bootstrap.ps1 [-SkipBuild] [-SkipClone] [-Help]

Options:
    -SkipBuild    Skip building tools from source
    -SkipClone    Skip cloning repositories
    -Help         Show this help message

Examples:
    .\bootstrap.ps1
    .\bootstrap.ps1 -SkipBuild
"@
}

function Test-Command {
    param([string]$Command)
    return [bool](Get-Command $Command -ErrorAction SilentlyContinue)
}

function Install-Mise {
    if (Test-Command "mise") {
        Write-Success "mise already installed: $(mise --version)"
        return
    }

    Write-Info "Installing mise..."

    if (Test-Command "winget") {
        winget install jdx.mise
    } elseif (Test-Command "scoop") {
        scoop install mise
    } else {
        Write-Warn "Please install mise manually: https://mise.run"
        Write-Info "  winget install jdx.mise"
        Write-Info "  # or: scoop install mise"
    }
}

function Clone-Repos {
    Write-Info "Cloning external tool repositories..."

    New-Item -ItemType Directory -Force -Path $ToolsDir | Out-Null

    $repos = @(
        "https://github.com/FlexNetOS/pixi.git",
        "https://github.com/FlexNetOS/nushell.git",
        "https://github.com/FlexNetOS/coreutils.git",
        "https://github.com/FlexNetOS/syn.git",
        "https://github.com/dan-t/rusty-tags.git",
        "https://github.com/twpayne/chezmoi.git"
    )

    Push-Location $ToolsDir
    try {
        foreach ($repo in $repos) {
            $name = [System.IO.Path]::GetFileNameWithoutExtension($repo)
            if (Test-Path $name) {
                Write-Info "  $name already cloned"
            } else {
                Write-Info "  Cloning $name..."
                git clone --depth 1 $repo 2>$null
                if ($LASTEXITCODE -ne 0) {
                    Write-Warn "  Failed to clone $name"
                }
            }
        }
    } finally {
        Pop-Location
    }

    Write-Success "Repositories ready"
}

function Build-Tools {
    Write-Info "Building tools from source..."

    $tools = @("nushell", "coreutils", "rusty-tags")

    foreach ($tool in $tools) {
        $toolPath = Join-Path $ToolsDir $tool
        if (Test-Path $toolPath) {
            Write-Info "  Building $tool..."
            Push-Location $toolPath
            try {
                cargo build --release 2>$null
                if ($LASTEXITCODE -eq 0) {
                    Write-Success "  $tool built"
                } else {
                    Write-Warn "  $tool build failed"
                }
            } finally {
                Pop-Location
            }
        }
    }

    # Chezmoi (Go)
    $chezmoiPath = Join-Path $ToolsDir "chezmoi"
    if ((Test-Command "go") -and (Test-Path $chezmoiPath)) {
        Write-Info "  Building chezmoi..."
        Push-Location $chezmoiPath
        try {
            git checkout v2.52.0 2>$null
            $env:GOPROXY = "direct"
            go build -o chezmoi.exe . 2>$null
            if ($LASTEXITCODE -eq 0) {
                Write-Success "  chezmoi built"
            } else {
                Write-Warn "  chezmoi build failed"
            }
        } finally {
            Pop-Location
        }
    }
}

function New-BinLinks {
    Write-Info "Creating portable bin directory..."

    New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

    # Create wrapper scripts (Windows doesn't have symlinks easily)

    # Nushell
    $nuExe = Join-Path $ToolsDir "nushell\target\release\nu.exe"
    if (Test-Path $nuExe) {
        $wrapper = Join-Path $BinDir "nu.cmd"
        "@echo off`n`"$nuExe`" %*" | Out-File -Encoding ASCII $wrapper
        Write-Success "  nu.cmd -> nushell"
    }

    # Chezmoi
    $chezmoiExe = Join-Path $ToolsDir "chezmoi\chezmoi.exe"
    if (Test-Path $chezmoiExe) {
        $wrapper = Join-Path $BinDir "chezmoi.cmd"
        "@echo off`n`"$chezmoiExe`" %*" | Out-File -Encoding ASCII $wrapper
        Write-Success "  chezmoi.cmd -> chezmoi"
    }

    # Rusty-tags
    $rustyTagsExe = Join-Path $ToolsDir "rusty-tags\target\release\rusty-tags.exe"
    if (Test-Path $rustyTagsExe) {
        $wrapper = Join-Path $BinDir "rusty-tags.cmd"
        "@echo off`n`"$rustyTagsExe`" %*" | Out-File -Encoding ASCII $wrapper
        Write-Success "  rusty-tags.cmd -> rusty-tags"
    }

    Write-Info "Add to PATH: `$env:PATH = `"$BinDir;`$env:PATH`""
}

function Install-PythonTools {
    Write-Info "Installing Python tools..."

    if (Test-Command "aider") {
        Write-Success "aider already installed"
    } else {
        Write-Info "  Installing aider..."
        pip install --user --upgrade aider-chat 2>$null
        if ($LASTEXITCODE -ne 0) {
            Write-Warn "  aider install failed"
        }
    }
}

function Set-AiderConfig {
    Write-Info "Configuring aider..."

    $configPath = Join-Path $ProjectRoot ".aider.conf.yml"

    @"
# Aider configuration for AgentAskit
# REF: ADR-0005 Modern Tooling Strategy

auto-commits: true
dirty-commits: false
attribute-author: true
gitignore: true
aiderignore: .aiderignore
dark-mode: true
pretty: true
stream: true
"@ | Out-File -Encoding UTF8 $configPath

    $ignorePath = Join-Path $ProjectRoot ".aiderignore"

    @"
# Aider ignore patterns
tools/external/
target/
*.lock
.git/
node_modules/
__pycache__/
*.pyc
.env
.env.local
"@ | Out-File -Encoding UTF8 $ignorePath

    Write-Success "aider configured"
}

function Show-Summary {
    Write-Host ""
    Write-Host "============================================" -ForegroundColor White
    Write-Host "  AgentAskit Development Environment Setup" -ForegroundColor White
    Write-Host "============================================" -ForegroundColor White
    Write-Host ""
    Write-Info "Tools directory: $ToolsDir"
    Write-Info "Bin directory:   $BinDir"
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor White
    Write-Host "  1. Add to your PowerShell profile:"
    Write-Host "     `$env:PATH = `"$BinDir;`$env:PATH`""
    Write-Host ""
    Write-Host "  2. For mise integration, add to profile:"
    Write-Host "     mise activate pwsh | Out-String | Invoke-Expression"
    Write-Host ""
    Write-Host "  3. Configure aider API key:"
    Write-Host "     `$env:ANTHROPIC_API_KEY = `"sk-ant-...`""
    Write-Host ""
}

# Main
function Main {
    if ($Help) {
        Show-Help
        return
    }

    Write-Host "AgentAskit Bootstrap Script (Windows)" -ForegroundColor White
    Write-Host "=====================================" -ForegroundColor White
    Write-Host ""

    # Check prerequisites
    if (-not (Test-Command "git")) {
        Write-Err "git not found. Please install git first."
        exit 1
    }
    if (-not (Test-Command "cargo")) {
        Write-Err "cargo not found. Please install Rust first."
        exit 1
    }

    Install-Mise

    if (-not $SkipClone) {
        Clone-Repos
    }

    if (-not $SkipBuild) {
        Build-Tools
    }

    New-BinLinks
    Install-PythonTools
    Set-AiderConfig

    Show-Summary

    Write-Success "Bootstrap complete!"
}

Main
