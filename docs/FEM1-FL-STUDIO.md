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

Keep both. The VST3 wrapper loads the CLAP engine and interface at runtime;
updating only the VST3 can leave the old interface in place. The original
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

### Embedded default firmware

New instances start with **M-VAVE stock V15**, embedded in the plug-in: no
download, internet access, or separate firmware file is required. Incoming DAW
MIDI is remapped to channel 2 by default because that is the stock firmware's
receive channel. Saved projects restore their own firmware and parameter values.
Use **Menu > Load firmware / save…** to choose another firmware, and adjust
**Advanced > MIDI channel override** if its routing differs.

The specific unmodified V15 recovery image is licensed by M-VAVE under Apache
2.0. Source provenance and original notices are included under `licenses/mvave`
in the package and `crates/fm1-plugin/resources/mvave` in this repository. This
does not establish redistribution rights for other stock firmware versions.

### Panel menu and colors

The plug-in's toolbar is now a **Menu** dropdown on the instrument panel. It holds
firmware import, portable save export, and **Theme**: Black, Green, Orange,
Purple, and White, inspired by the [linked product listing](https://www.amazon.com/dp/B0H5VCT7XK).
The finish is saved per plug-in instance in the DAW project. It does not change
firmware, sound, LCD contents, or the firmware's LED states. FM-1 case finishes
apply to the FM-1 panel; SMK-37 firmware retains its own keyboard panel.

The plug-in WAV recorder has been removed; record and render using the DAW.
The panel's hardware **REC** key still goes to the firmware's sequencer.

**Menu > Advanced** retains MIDI channel override (new-instance default: 2),
emulated CPU clock (default: 240 MHz), and audio safety buffer / Lead (default:
23 ms). MIDI routing belongs in the DAW normally, but some firmware expects a
specific channel. CPU clock changes the emulated hardware's speed and restarts
the firmware. Lead is extra scheduling time for the emulator, beyond the host
block; lowering it reduces live latency while increasing dropout risk. These
settings remain available for compatibility and performance troubleshooting.

For an update, close FL Studio before running the extracted package's
`Install.ps1 -SystemInstall` in an administrator PowerShell. Replace both the
VST3 and CLAP companion. Reopen FL Studio and rescan if it still shows the old UI.

### Host checks

### Linking panel knobs to a controller

Master, Select, Presets, Algorithm, and Knobs 1–4 are host parameters. Move a
knob in the plug-in, then choose **Tools > Last tweaked > Link to controller** in
FL Studio and move the hardware control. Alternatively use the wrapper's
**Browse parameters** list. Rescan with Verify plugins if the old parameter list
is cached. See [Image-Line's controller-linking guide](https://www.image-line.com/fl-studio-learning/fl-studio-online-manual/html/automation_linking.htm).

The encoder parameters use virtual positions 0–127 (starting at 64). Their
movement becomes relative encoder clicks, so their meaning depends on the
firmware and its active page. They do not expose a universal absolute filter,
envelope, or preset value. Mouse turning remains endless at the host position's
end stops. Saved firmware/flash remains the basis for recalling sound state;
restoring a project rebases the encoder positions without replaying old turns.
An eighth encoder parameter supports the SMK-37 panel.

Stock V15's OCT− and HOME illumination changes were reproduced with no input.
The changes are already present in emulated GPIO LED duty cycles, rather than
being synthesized by the editor or generated by unsolicited key commands. The
firmware LED display is retained. Comparison with a physical unit remains open.

### Playback checklist

1. Save any current FL Studio project before testing. Install to the system
   locations above. Open **Options > File settings > Manage plugins** and run
   **Find installed plugins** with **Verify plugins** enabled. Look for
   **F'Em 1** / **FEm1**, vendor **ItsRobbAllen**.
2. Add the VST3 as an instrument. Open **Menu > Load firmware / save…** and select a
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
