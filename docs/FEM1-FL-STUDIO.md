# F'Em 1 — first Windows / FL Studio test

F'Em 1 is based on fm1-emu by Joakim Eriksson. Its first prototype retains the
upstream emulator and firmware-loading interface while using a distinct plug-in
identity: `io.github.itsrobballen.fem1`. The display name is **F'Em 1**, by
**ItsRobbAllen**. Build artifacts use `FEm1` for simple filenames.

## Build

Run `tools\build_plugin.ps1` with Rust (MSVC), Visual Studio C++ Build Tools,
CMake, and Git available. It installs both files under your user profile:

- `%LOCALAPPDATA%\Programs\Common\VST3\FEm1.vst3` (the entire bundle folder)
- `%LOCALAPPDATA%\Programs\Common\CLAP\FEm1.clap`

Keep both. The VST3 wrapper loads the CLAP engine at runtime. The original
`FM-1` plug-in uses different filenames and an independent identity.

For FL Studio, run `tools\build_plugin.ps1 -SystemInstall` from an administrator
PowerShell to install to `C:\Program Files\Common Files\VST3` and
`C:\Program Files\Common Files\CLAP`. Image-Line's manual requires Windows VST3
plug-ins in the standard system locations; adding a custom search path alone
may not find them. See [Image-Line's installation guide](https://www.image-line.com/fl-studio-learning/fl-studio-online-manual/html/basics_externalplugins.htm).

If already built, use `tools\install_plugin.ps1 -SystemInstall` instead; no rebuild
is needed. The Windows zip also includes `Install.ps1`; extract the zip and run
`.\Install.ps1 -SystemInstall` from an administrator PowerShell in that folder.

## FL Studio acceptance checklist

1. Save any current FL Studio project before testing. Install to the system
   locations above. Open **Options > File settings > Manage plugins** and run
   **Find installed plugins** with **Verify plugins** enabled. Look for
   **F'Em 1** / **FEm1**, vendor **ItsRobbAllen**.
2. Add the VST3 as an instrument. Open **Load firmware / save…** and select a
   `.fwsc` package obtained from its author. No firmware is bundled here.
3. Play the panel with the mouse, then play MIDI from FL Studio's piano roll.
   Check the firmware's MIDI channel; M-VAVE's stock firmware listens on 2.
4. Save a small FL Studio project. Close and reopen it. Verify that the same
   firmware and its saved patches return. Save patches through the firmware's
   own controls before saving the DAW project.
5. Render a short WAV and compare it with live playback: notes should be
   present, correctly timed, and free of obvious dropouts. Test 44.1 and 48 kHz.
6. Load two instances with different firmware and confirm they remain independent.

Record FL Studio version, firmware/version, audio driver, sample rate, buffer
size, and any failed step in PROGRESS.md. A successful build or validator run
does not establish FL Studio compatibility; these host checks are still required.
