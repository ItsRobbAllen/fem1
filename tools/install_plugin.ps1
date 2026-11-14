# Install an already-built F'Em 1. For FL Studio, run with -SystemInstall
# from an administrator PowerShell. Also works beside the packaged plug-ins.
param(
    [switch]$SystemInstall,
    [string]$SourceDirectory = ''
)
$ErrorActionPreference = 'Stop'
if (-not $SourceDirectory) {
    $SourceDirectory = if (Test-Path -LiteralPath (Join-Path $PSScriptRoot 'FEm1.clap')) {
        $PSScriptRoot
    } else { Join-Path $PSScriptRoot '..\target\plugin' }
}
$source = [IO.Path]::GetFullPath($SourceDirectory)
foreach ($artifact in 'FEm1.clap', 'FEm1.vst3\Contents\x86_64-win\FEm1.vst3') {
    if (-not (Test-Path -LiteralPath (Join-Path $source $artifact) -PathType Leaf)) {
        throw "Missing built artifact: $source\$artifact"
    }
}
$destination = Join-Path $env:LOCALAPPDATA 'Programs\Common'
if ($SystemInstall) {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = [Security.Principal.WindowsPrincipal]::new($identity)
    if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        throw 'System installation needs an administrator PowerShell. Run this script there with -SystemInstall.'
    }
    $destination = Join-Path $env:ProgramFiles 'Common Files'
}
foreach ($format in 'VST3', 'CLAP') {
    $parent = [IO.Path]::GetFullPath((Join-Path $destination $format))
    $artifact = if ($format -eq 'VST3') { 'FEm1.vst3' } else { 'FEm1.clap' }
    $target = [IO.Path]::GetFullPath((Join-Path $parent $artifact))
    if (-not $target.StartsWith($parent.TrimEnd('\') + '\', [StringComparison]::OrdinalIgnoreCase)) {
        throw "Invalid install destination: $target"
    }
    New-Item -ItemType Directory -Force -Path $parent | Out-Null
    if (Test-Path -LiteralPath $target) { Remove-Item -LiteralPath $target -Recurse -Force }
    Copy-Item -LiteralPath (Join-Path $source $artifact) -Destination $target -Recurse
    Write-Host "Installed $target"
}
