param(
  [string]$Root = "."
)
$ErrorActionPreference = 'Stop'
$rootPath = (Resolve-Path $Root).Path

& "$rootPath/agentaskit-production/tools/analysis/cross_reference.ps1" -Root $rootPath -OutDir "$rootPath/agentaskit-production/docs/reports/cross_reference/artifacts" | Out-Host
& "$rootPath/agentaskit-production/tools/analysis/update_todo_from_report.ps1" | Out-Host

# Local-only runners (no network)
$llRun = "$rootPath/agentaskit-production/integrations/llama.cpp/run_many.ps1"
if (Test-Path $llRun) { & $llRun }

Write-Host "Local run_all completed (offline mode)." -ForegroundColor Green
