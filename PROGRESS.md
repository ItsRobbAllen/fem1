# F'Em 1 progress

## 2026-11-14 — Repository and checkpoint setup

- Foundation: joakimeriksson/fm1-emu at `31d9ac0`, including the fm1-sid submodule.
- Product name: **F'Em 1**; repository/internal name: `fem1`.
- Local development branch: `fem1`; original repository retained as `upstream`.
- Target fork: https://github.com/itsrobballen/fem1.
- Added AGENTS.md with milestone commits, pushes, and progress notes so work can
  resume across sessions. Aim for checkpoints about every 30 minutes during
  longer active work, and before ending a session.

Validation: clean starting checkout; submodule initialized at its recorded commit.
Documentation-only setup; no application behavior changed or build run.

Fork verified at https://github.com/ItsRobbAllen/fem1 (renamed from fm1-emu).
Origin points to this fork; upstream remains the original repository.
GitHub connector confirms write access. Local Git authentication still needs a
refresh (saved credential returned HTTP 401); use the connector to publish
checkpoints until local Git authentication is restored.

## Next steps

1. Verify Windows VST3 build prerequisites and build the existing plug-in.
2. Evaluate a synth, groovebox, and sampler firmware for sound, CPU use, timing,
   project recall, and offline rendering before deeper product changes.

## 2026-11-14 — First F'Em 1 Windows VST3 prototype

- Installed Rust stable/MSVC, Visual Studio 2022 C++ Build Tools, and CMake.
- Display name/vendor: F'Em 1 / ItsRobbAllen. Independent CLAP ID:
  `io.github.itsrobballen.fem1`; VST3 class ID:
  `E6DC9221DD6852F5A09E61D26CAC6179`.
- Updated wrapper identities, build/package scripts, and release validation paths.
  Retained upstream licensing and the emulator/firmware behavior.
- Added install-only script, with an administrator preflight for system installs,
  and an FL Studio acceptance checklist in docs/FEM1-FL-STUDIO.md.
- Built `target/plugin/FEm1.vst3` and `target/plugin/FEm1.clap`.
- Packaged `target/plugin/release/fem1-plugin-0.5.1-windows-x86_64.zip`, including
  an Install.ps1 that works without build tools.

Validation:

- Release plug-in tests: 14 unit tests, 2 engine tests, and 2 portable-save tests
  passed. Felucca 1.8.2 was downloaded from its author for the engine tests.
- Steinberg VST3 validator: **47 passed, 0 failed**; output in
  `target/plugin/validator.log`. Descriptor name, vendor, and class ID verified.
- PowerShell script syntax and Git whitespace checks passed.
- Per-user install succeeded at
  `C:/Users/mspas/AppData/Local/Programs/Common/{VST3,CLAP}`.
- System install was denied by Windows permissions. For FL Studio, run
  `tools/install_plugin.ps1 -SystemInstall` in an administrator PowerShell, or
  extract the package and run its `Install.ps1 -SystemInstall` there.
- FL Studio scanning, MIDI/audio, project recall, offline rendering, and two
  instances still need host testing. No FL Studio compatibility claim yet.
- macOS identity/build script updates were reviewed but not built on this Windows
  machine. Existing emulator compiler warnings remain.

Next: install in the system folders and run the FL Studio checklist. This is an
initial renamed prototype (version inherited from upstream: 0.5.1), not a new
public release. Broader synth/groovebox/sampler qualification remains pending.

## 2026-11-14 — Panel menu, finishes, and DAW recording

- Removed the top toolbar. Added an on-panel Menu for firmware import/export,
  case finishes, and Advanced controls. File work keeps running with menus closed.
- Added Black, Green, Orange, Purple, and White finishes from the user's linked
  Amazon listing (approximate hardware-inspired palettes). Each instance saves
  its finish in DAW state; old projects default to Black. LCD and LED states are
  unaffected. SMK-37 retains its dedicated panel.
- Removed the plug-in's WAV recording command and engine path. The hardware REC
  button remains a firmware input. Recording/rendering is handled by the DAW.
- Kept MIDI override, CPU Clock, and Lead in Advanced. Existing parameter IDs and
  defaults remain compatible with saved projects and automation.
- Tests: all 20 release plug-in tests passed, including new finish persistence,
  legacy-state fallback, LCD preservation, and pressed-key feedback checks.
  All 47 Steinberg VST3 validator checks passed with the new CLAP engine in an
  isolated preview folder (to avoid accidentally loading the installed old build).
- Native preview showed the toolbar removed and theme rendering, but the SDK
  editor host clips the panel at this display scaling; full dropdown interaction
  and visual verification remain for FL Studio. No user FL Studio project edited.
- Build/package completed locally. Updated package is
  `target/plugin/release/fem1-plugin-0.5.1-windows-x86_64-colors.zip`.
  Close FL Studio and reinstall both companion files with administrator rights
  before testing. The active installed plug-in was not replaced while FL Studio
  was running.

Next: verify menu operation, all five finishes, and reopen a saved FL Studio
project to check the selected finish and firmware together.

## 2026-11-14 — Embedded stock firmware

- User confirmed the menu/colors update worked after installing both companion
  files through the installer. Copying only the VST3 had loaded the old CLAP UI.
- Verified M-VAVE's explicit Apache-2.0 grant for the unmodified V15 recovery
  image at m-vave/fm1_open commit `4c7496daac04a95ff10d1afdcc8f9ead2dbabc41`.
  Bundled that exact 699,956-byte image, original notices, and Apache license in
  resources/mvave. This grant is specific to V15, not the separate V16 download.
- New instances use the embedded image without disk reads or network requests;
  instance firmware copies share immutable package bytes. Saved project firmware
  still overrides the default. The editor also boots its loaded firmware when a
  host displays the UI without activating audio.
- New-instance MIDI override defaults to channel 2 for stock V15 to play from
  ordinary DAW MIDI. Saved parameter values keep their own routing. Other
  firmware can be loaded through Menu as before.
- Validation: 22 release tests passed; an additional embedded-stock boot/screen/
  MIDI-audio test passed. Pinned image hash/parser and project firmware override
  are covered. The rebuilt VST3 passed all 47 validator checks using an isolated
  CLAP filename to avoid the previously installed version.
- Updated package includes notices under licenses/mvave; zip contents verified.
  `target/plugin/release/fem1-plugin-0.5.1-windows-x86_64-bundled.zip`.
  Close FL Studio and run its extracted Install.ps1 -SystemInstall to update both
  files. No active FL Studio project or installed binary was modified this turn.

Next: open a fresh instance in FL Studio and confirm the stock synth appears and
plays immediately, then reopen a project using a different firmware to verify
recall in the DAW.

## 2026-11-14 — Theme naming and controller-linkable encoders

- Renamed the visible Case color menu to Theme; kept the saved-state key intact.
- Confirmed only Master was exposed among panel knobs. Added stable host IDs for
  Select, Algorithm, Presets, Knobs 1–4, and SMK's eighth encoder. UI gestures
  notify the host for Last tweaked linking; automation works with the editor shut.
- Encoder values are virtual positions 0–127 converted to relative hardware
  movements. Mouse turns remain endless at end stops. Firmware boot and clock
  reboot establish a baseline, preventing saved encoder positions replaying as
  new turns. Sound meaning remains firmware/page dependent.
- Reproduced stock V15 OCT−/HOME illumination changes with no input in a temporary
  diagnostic test. Raw 30 ms GPIO-duty samples alternated around 0.018 and 0.090;
  the UI does not invent these changes or send key presses for them. Kept LED
  output unchanged; physical-device comparison remains unverified. Diagnostic
  test was removed; its log remains in target/plugin/led-probe.log.
- Validation: all 24 release tests passed. All 47 VST3 validator checks passed;
  its Scan Parameters output confirms the newly exposed names and IDs. Host MIDI
  controller linking still needs user testing in FL Studio.
- New package: target/plugin/release/fem1-plugin-0.5.1-windows-x86_64-controls.zip.
  Close FL Studio, extract, and run Install.ps1 -SystemInstall as administrator
  to replace both files, then rescan with Verify plugins as needed.

Next: move a panel knob, use FL Studio Tools > Last tweaked > Link to controller,
and move the hardware knob. Check forward/reverse movement on the intended
firmware page and project recall. The stock firmware remains embedded.
