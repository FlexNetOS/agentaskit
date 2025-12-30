param(
  [string]$Config = "agentaskit-production/integrations/llama.cpp/config/models.yaml"
)
$ErrorActionPreference = 'Stop'

function Try-LoadYaml {
  param([string]$Path)
  try {
    if (Get-Module -ListAvailable -Name powershell-yaml) {
      Import-Module powershell-yaml -ErrorAction Stop | Out-Null
      return (Get-Content -Raw -Path $Path | ConvertFrom-Yaml)
    }
  } catch {}
  return $null
}

$confPath = (Resolve-Path $Config).Path
$primary = $env:LLAMACPP_PRIMARY_3B
$secondary = $env:LLAMACPP_SECONDARY_7B
$count = $env:LLAMACPP_STACKS_COUNT
$prompt = $env:LLAMACPP_PROMPT
$threads = $env:LLAMACPP_THREADS

if (-not ($primary -and $secondary -and $count -and $prompt -and $threads)) {
  $cfg = Try-LoadYaml -Path $confPath
  if ($cfg -ne $null) {
    if (-not $primary) { $primary = $cfg.models.primary_3b.path }
    if (-not $secondary) { $secondary = $cfg.models.secondary_7b.path }
    if (-not $count) { $count = [int]$cfg.stacks.count }
    if (-not $prompt) { $prompt = $cfg.stacks.prompt }
    if (-not $threads) { $threads = [int]$cfg.stacks.threads }
  }
}

if (-not $count) { $count = 1 }
if (-not $prompt) { $prompt = 'ping' }
if (-not $threads) { $threads = 4 }

$root = (Split-Path $confPath -Parent)
$exe = Join-Path $root 'llama.cpp/main.exe'
if (-not (Test-Path $exe)) { $exe = Join-Path $root 'llama.cpp/build/bin/main.exe' }
if (-not (Test-Path $exe)) { Write-Warning 'llama.cpp main executable not found; skip'; exit 0 }

for ($i=0; $i -lt [int]$count; $i++) {
  if ($primary -and (Test-Path $primary)) { Start-Process -FilePath $exe -ArgumentList @('-m', $primary, '-p', $prompt, '-t', $threads) }
  if ($secondary -and (Test-Path $secondary)) { Start-Process -FilePath $exe -ArgumentList @('-m', $secondary, '-p', $prompt, '-t', $threads) }
}
Write-Host "Started $count stacks (primary 3B, secondary 7B if present)."
