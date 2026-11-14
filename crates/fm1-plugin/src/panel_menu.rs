use crate::panel;

/// A software-only key, painted with the instrument's own panel primitives.
pub fn draw(buf: &mut [u32], width: usize, y: f32, p: panel::Palette, down: bool, hover: bool) {
    panel::canvas(width);
    let x = width as f32 - 164.0;
    panel::rrect(buf, x - 5.0, y - 5.0, 146.0, 57.0, 13.0, panel::TRAY, panel::TRAY);
    let off = if down { 2.0 } else { 0.0 };
    panel::rrect(buf, x, y + 4.0, 136.0, 44.0, 9.0, 0x17181b, 0x17181b);
    let light = if hover && !down { 0.10 } else { 0.0 };
    panel::rrect(buf, x, y + off, 136.0, 44.0, 9.0,
        panel::mix(p.btn_top, 0xffffff, light), panel::mix(p.btn_bot, 0xffffff, light));
    panel::text(buf, x + 57.0, y + 15.0 + off, "MENU", 2, p.btn_ink);
    // A small recessed down arrow, using the same ink as the button legends.
    for row in 0..4 {
        for col in row..7-row {
            panel::put(buf, (x + 108.0) as i32 + col, (y + 20.0 + off) as i32 + row, p.btn_ink, 1.0);
        }
    }
}
