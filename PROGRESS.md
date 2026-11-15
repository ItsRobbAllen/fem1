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

### 2026-11-14 — case-matched menu button

- Replaced the generic hamburger/menu widget with a panel-rendered MENU key:
  matching pixel lettering, rounded cap, recessed surround, shadow, down arrow,
  and theme colors. Added hover/pressed feedback and proportional scaling.
- Kept the existing dropdown and its actions; the transparent interaction layer
  cannot send firmware key presses. Reviewed rendered previews of all five themes
  against the adjacent physical keys (target/plugin/menu-preview.png).
- Validation: 24 release tests and all 47 VST3 validator checks passed; rebuilt
  both plug-in files. Packaged theme-controls.zip with the embedded stock firmware
  and encoder linking changes. FL Studio controller linking needs user testing.
- Checkpoint publishing uses the GitHub connector because saved local Git
  credentials were rejected; local milestone history is retained before fetching
  and tracking the equivalent remote commits.

Next: close FL Studio, extract the theme-controls package, run its Install.ps1
with the same installation scope as before, then try the new menu and encoder
linking. OCT−/HOME emulated LED behavior remains unchanged as documented above.

### 2026-11-14 — realistic panel design preview

- Created a code-rendered SVG concept matching the supplied hardware reference:
  fine-grain dark shell, rim highlights, smaller ridged knobs, molded rubber
  controls, recessed trays, glass display and OP1–OP6/PIT/GLO/MONO/POLY legends.
- The MENU key remains in the case style. LCD content is illustrative, not a
  live firmware capture. Printed shortcut behavior is not yet verified.
- Preview only: no plug-in code changes, compilation or installation. Inspected
  the rendered PNG at target/ui-preview/realistic-panel-preview.png. The editable
  SVG and generator are in docs/design; render with node preview-panel.cjs --png
  after installing @resvg/resvg-js under target/ui-preview.
- Published design checkpoint through GitHub connector; local history retained.

Next: get user feedback on this visual direction before implementing the new
renderer, updating hit targets, and validating firmware shortcut behavior.

### 2026-11-14 — approved realistic panel implemented

- Embedded the approved panel artwork in five theme variants: textured shell,
  beveled rim, recessed trays, molded keys, ridged knobs and printed shortcut
  legends. Assets total about 1.6 MB; no external artwork or fonts are needed.
- New plug-in renderer replaces the sample screen with the live RGB565 firmware
  framebuffer, rotates encoder/master caps, and renders pressed/LED feedback.
  Existing matrix IDs, automation IDs and saved theme values are preserved.
- Matched every new control hit target to its artwork. Added exhaustive center
  checks for 27 keys, 14 buttons and 8 knobs, plus embedded-theme/LCD/pressed-key
  rendering tests. New windows default to the artwork's aspect ratio. Existing
  saved window sizes fit the panel proportionally. SMK retains its old renderer.
- Validation: 26 release tests and all 47 VST3 validator checks passed. Inspected
  actual booted firmware renders in all five themes (target/plugin/realistic-preview).
  This is renderer/validator testing; interactive FL Studio testing remains.
- Built and packaged target/plugin/release/fem1-plugin-0.5.1-windows-x86_64-realistic.zip
  with both VST3 and CLAP, installer and firmware notices. Did not replace plug-in
  files while the user's DAW is running. Close FL Studio and use the same installer
  scope as the previous working installation.
- Published through GitHub Git-object tools, with matching local/remote tree and
  a backup branch retaining local checkpoint history. Build tooling instructions
  and the optional live-firmware visual QA path are in docs/design/README.md.

Next: install the realistic package and verify pointer alignment, popup actions,
knob linking and theme/project recall in FL Studio. Shortcut legends preserve
existing physical key positions; their firmware behavior is not hardware-verified.
OCT−/HOME LED timing remains unchanged.

### 2026-11-14 — stationary knob lighting, endless MIDI encoders and labels

- Removed baked pointer strokes from all theme assets. The renderer paints only
  the rotating antialiased indicator over stationary reflections, knurling and
  shadows. Corrected the master's absolute pointer angle after this separation.
- Removed the center F'Em 1 mark and changed the bottom legend to exactly
  "F'Em 1 - the Virtual Virtual FM Synthesizer" in all themes.
- Identified the bounded host mapping: 0–127, initially 64, permits only 63 upward
  movements. Kept existing host parameter IDs/ranges for project compatibility;
  added Menu > MIDI encoders for direct binary-offset relative CC learning,
  including the user's new FLkey 2 controller format (63=-1 / 65=+1).
- Assignments persist by actual input channel and CC; learning/visual counters
  do not persist. Learned CCs are consumed, other events continue to firmware.
  Notes retain their original channel-override path; CC override happens after
  learning on the machine thread. No new locks or allocations in audio callback.
- Patched pinned clap-wrapper v0.16.0 to emit every queued MIDI point, instead of
  dropping all but the last CC in each block. The checked-in patch applies on
  CMake configure, recognizes an already patched source, and fails on mismatch.
  Verified fresh patch application and repeated-configuration detection.
- Validation: all 30 release tests passed, including 1,000 repeated forward and
  1,200 reverse relative events, binding/state recall, stationary-lighting pixel
  checks, and actual firmware preset-number changes beyond 63 turns and on
  reversal. Rebuilt both artifacts; all 47 VST3 validator checks passed.
- Updated FL Studio guide with official Novation configuration and matching
  input-port instructions. Included FL-STUDIO.md in the Windows package:
  target/plugin/release/fem1-plugin-0.5.1-windows-x86_64-encoders.zip.
- Reviewed final artwork/legend. Published checkpoint through GitHub Git-object
  tools and retained local history in a backup branch. The user's running DAW
  installation was not overwritten.

Next: close FL Studio and install both files with the package installer. For
endless browsing, configure FLkey 2 Custom mode as 7-bit relative MIDI CC,
remove the old bounded Presets link, route Custom-mode CCs to this instance,
and use Menu > MIDI encoders > Presets > Learn. Physical controller/FL Studio
routing has not been exercised here. Preset wrap/acceleration remains firmware
behavior; the OCT−/HOME LED output remains unchanged.

### 2026-11-14 — FM-106 package for local loading

- Downloaded the catalog-listed FM-106 1.0 package to the user's chosen folder:
  reference/firmwares/fm106.fwsc, 611486 bytes, identity FM-1_907.
  Source: https://thefirmwarehub.com/fw/fm106-1.0.fwsc
  Upstream: https://github.com/PatttF/fm-106
  SHA256: 41746d625216f1647964c7d8993be3e0b9e1950b415288e723603fa5f8c45ca6.
- Initial download-check code passed a Node Buffer into a parser whose slice
  operations mutate Buffer views. Corrected it to validate a copied Uint8Array
  before saving pristine downloaded bytes. Replaced the affected files; removed
  redundant copies created in this session, retaining the single working package.
- Native package parsing passed. The existing plug-in engine's UI-only boot test
  passed with FM1_TEST_FW=fm106: 35678 nonzero LCD pixels, 2.8x headroom. Log:
  target/plugin/fm106-boot-check.log. This verifies startup/display, not MIDI/audio
  performance across every patch. No plug-in rebuild or USB hardware required.
- Firmware stays local/ignored; only this progress record is checkpointed.

Next: select reference/firmwares/fm106.fwsc via Menu > Load firmware / save…
in F'Em 1, then play it in FL Studio and save the project to retain that firmware.
