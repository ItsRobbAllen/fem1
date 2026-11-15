# Zip the plugin tools\build_plugin.ps1 built (target\plugin) for a release:
# target\plugin\release\fem1-plugin-<version>-windows-x86_64.zip with the VST3 (a folder), the CLAP
# and an install guide. The build is not code-signed; Windows hosts load it as it is.
#   tools\package_plugin.ps1
$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')
$out = 'target\plugin'
$version = (Select-String -Path 'crates\fm1-plugin\Cargo.toml' -Pattern '^version = "(.*)"' |
    Select-Object -First 1).Matches[0].Groups[1].Value
if (-not $version) { throw 'no version in crates\fm1-plugin\Cargo.toml' }
$name = "fem1-plugin-$version-windows-x86_64"
$rel = "$out\release"
Remove-Item $rel -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force "$rel\$name" | Out-Null
foreach ($b in 'FEm1.vst3', 'FEm1.clap') {
    if (-not (Test-Path "$out\$b")) { throw "no $out\${b}: run tools\build_plugin.ps1 first" }
    Copy-Item "$out\$b" "$rel\$name\$b" -Recurse
}
Copy-Item -LiteralPath 'tools\install_plugin.ps1' -Destination "$rel\$name\Install.ps1"
Copy-Item -LiteralPath 'docs\FEM1-FL-STUDIO.md' -Destination "$rel\$name\FL-STUDIO.md"
New-Item -ItemType Directory -Force "$rel\$name\licenses\mvave" | Out-Null
foreach ($notice in 'NOTICE.md', 'THIRD_PARTY.md', 'LICENSE-2.0.txt') {
    Copy-Item -LiteralPath "crates\fm1-plugin\resources\mvave\$notice" -Destination "$rel\$name\licenses\mvave\$notice"
}

$install = @"
F'Em 1 plugin ${version}: the M-VAVE FM-1 emulator as an instrument (VST3, CLAP)
https://github.com/ItsRobbAllen/fem1

For Windows 10 or 11, 64-bit (x86-64).

For FL Studio: extract this zip, open an administrator PowerShell in the extracted
folder, then run: .\Install.ps1 -SystemInstall
This installs the existing build without requiring build tools.

1. Copy both plug-ins to your plug-in folders (create the folders that are missing):
     FEm1.vst3  (the whole folder)  ->  C:\Program Files\Common Files\VST3\
     FEm1.clap                      ->  C:\Program Files\Common Files\CLAP\
   Without administrator rights, the same folders under your own profile work too:
     C:\Users\<you>\AppData\Local\Programs\Common\VST3\  and  ...\Common\CLAP\
   (For FL Studio use the Program Files locations above.)
   Keep both: the VST3 loads FEm1.clap from the CLAP folder when a DAW opens it.
2. This build is not code-signed. Windows may warn when you open the zip; the plug-ins
   themselves load without a prompt.
3. Restart your DAW, or rescan its plug-ins. Add "F'Em 1" (by ItsRobbAllen) as an instrument on a track.
4. New instances start with embedded M-VAVE stock V15; no firmware download is needed.
   To change it, use Menu > Load firmware / save... and pick a .fwsc package: a community
   firmware (the list, with each author's link: https://joakimeriksson.github.io/fm1-emu/)
   or M-VAVE's own. Play it from MIDI or with the mouse on its panel.

The firmware and the patches it saves are kept in your project.
"@
Set-Content -Path "$rel\$name\INSTALL.txt" -Value $install

Compress-Archive -Path "$rel\$name" -DestinationPath "$rel\$name.zip"
Remove-Item "$rel\$name" -Recurse -Force
Write-Host "packaged $rel\$name.zip (unsigned)"
