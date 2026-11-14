# Realistic panel

The approved SVG concept is the source for the plug-in's embedded panel artwork.
It is original code-drawn artwork informed by the supplied FM-1 product photo;
the photo itself is not embedded. Gradients, geometry and texture are generated
by `preview-panel.cjs`. The plug-in does not run JavaScript or render SVG at runtime.

From the repository root, regenerate the five indexed PNG assets with:

```powershell
npm install --prefix target/ui-preview --no-audit --no-fund @resvg/resvg-js@2.6.2 sharp@0.35.5
node docs/design/preview-panel.cjs --assets
```

The script uses system Arial for the rasterized legends. The checked-in PNGs
ensure builds render consistently and require neither Node nor installed fonts.
`--png` writes the illustrative concept to `target/ui-preview`. The illustrative
screen is removed from every embedded theme; the Rust renderer fills that area
from the live 240×240 RGB565 firmware framebuffer.

`realistic_panel.rs` owns the matching hit geometry and dynamic rendering. It
rotates knob caps, shades held keys, and tints caps from firmware LED output.
Hardware matrix IDs and encoder parameter IDs are unchanged. Printed shortcut
legends identify the same keyboard pads as the physical reference; their
firmware-specific behavior has not been independently compared with hardware.

For visual QA with actual booted firmware, set `FEM1_PANEL_PREVIEW` to an output
directory and run the stock boot test. This emits one rendered PNG per theme.
The alternate SMK board continues to use its existing renderer and controls.
