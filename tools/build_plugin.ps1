# Build the FEm1 plugin (Windows, x86-64): the CLAP from Rust (crates/fm1-plugin), then the VST3
# around it (clap-wrapper, crates/fm1-plugin/wrap). Unlike the macOS bundles, the Windows VST3
# doesn't carry the CLAP: it loads FEm1.clap at run time from a CLAP folder (Common Files\CLAP,
# %LOCALAPPDATA%\Programs\Common\CLAP, or a folder in CLAP_PATH), so both files are installed.
#   tools\build_plugin.ps1             # build, install in %LOCALAPPDATA%\Programs\Common\{VST3,CLAP}
#   tools\build_plugin.ps1 -SystemInstall # standard system folders for FL Studio (administrator)
#   $env:NO_INSTALL = 1; tools\build_plugin.ps1   # only build, into target\plugin\
# Needs Rust (MSVC), Visual Studio Build Tools (C++ workload), CMake and git:
#   winget install Rustlang.Rustup Kitware.CMake Git.Git
#   winget install Microsoft.VisualStudio.2022.BuildTools --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
param([switch]$SystemInstall)
$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')
$root = (Get-Location).Path
$out = Join-Path $root 'target\plugin'
New-Item -ItemType Directory -Force $out | Out-Null

# Only remove our named artifacts inside the intended output/install directory.
function Remove-PluginArtifact([string]$Path, [string]$Parent) {
    $resolved = [IO.Path]::GetFullPath($Path)
    $allowed = [IO.Path]::GetFullPath($Parent).TrimEnd('\') + '\'
    if (-not $resolved.StartsWith($allowed, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Artifact path is outside ${Parent}: $resolved"
    }
    if (Test-Path -LiteralPath $resolved) {
        Remove-Item -LiteralPath $resolved -Recurse -Force
    }
}

# the CLAP: the cdylib, renamed
Push-Location crates\fm1-plugin
cargo build --release
if ($LASTEXITCODE) { exit $LASTEXITCODE }
Pop-Location
$clap = Join-Path $out 'FEm1.clap'
Copy-Item crates\fm1-plugin\target\release\fm1_plugin.dll $clap -Force

# the VST3 around it (CMake's default generator on Windows is the newest Visual Studio it finds,
# Build Tools included: 2022 here, 2026 on GitHub's windows-latest, so no version is named)
cmake -S crates\fm1-plugin\wrap -B "$out\build" -A x64 -DFM1_CLAP="$clap"
if ($LASTEXITCODE) { exit $LASTEXITCODE }
cmake --build "$out\build" --config Release
if ($LASTEXITCODE) { exit $LASTEXITCODE }
$vst3 = Get-ChildItem "$out\build\Release" -Directory -Filter 'FEm1.vst3' | Select-Object -First 1
if (-not $vst3) { throw "no FEm1.vst3 under $out\build\Release" }
Remove-PluginArtifact "$out\FEm1.vst3" $out
Copy-Item $vst3.FullName "$out\FEm1.vst3" -Recurse
Write-Host "built $clap and $out\FEm1.vst3"

if ($env:NO_INSTALL) { exit 0 }
& (Join-Path $PSScriptRoot 'install_plugin.ps1') -SourceDirectory $out -SystemInstall:$SystemInstall
