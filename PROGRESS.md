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
