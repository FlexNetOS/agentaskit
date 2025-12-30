param(
  [string]$Repo = "$env:GITHUB_REPOSITORY"
)
$ErrorActionPreference = 'Stop'
if (-not $Repo) { throw "Set GITHUB_REPOSITORY (e.g. owner/repo) or pass -Repo" }
$token = $env:GITHUB_TOKEN
if (-not $token) { throw "Set GITHUB_TOKEN with repo:write permissions" }
$uri = "https://api.github.com/repos/$Repo/dispatches"
$body = @{ event_type = 'run-todo' } | ConvertTo-Json
Invoke-RestMethod -Method Post -Uri $uri -Headers @{ Authorization = "Bearer $token"; 'Accept' = 'application/vnd.github+json' } -Body $body
Write-Host "repository_dispatch run-todo sent to $Repo"
