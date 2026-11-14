//! Plug-in-only panel. Geometry follows the approved hardware-inspired artwork.
use crate::{appearance::CaseColor, panel};
pub const W: usize = 1500;
pub const H: usize = 920;
pub const MENU: (usize, usize, usize, usize) = (1264, 489, 121, 42);
pub const ENCS: [(f32, f32); 7] = [
    (281., 155.),
    (126., 312.),
    (281., 312.),
    (839., 155.),
    (1008., 155.),
    (1177., 155.),
    (1346., 155.),
];
const WHITE: [u8; 16] = [0, 2, 4, 6, 7, 9, 11, 12, 14, 16, 18, 19, 21, 23, 24, 26];
const BLACK: [u8; 11] = [1, 3, 5, 8, 10, 13, 15, 17, 20, 22, 25];
const UPPER: [usize; 11] = [123, 207, 291, 460, 544, 713, 797, 881, 1050, 1134, 1303];

pub fn buttons() -> Vec<(u8, usize, usize, usize, usize)> {
    let mut result = vec![(0, 101, 406, 99, 53), (1, 214, 406, 101, 53)];
    for i in 0..12 {
        result.push((i as u8 + 2, 813 + i % 6 * 95, 284 + i / 6 * 91, 79, 78));
    }
    result
}
pub fn keys() -> Vec<(u8, usize, usize, usize, usize)> {
    let mut result = Vec::with_capacity(27);
    for (i, n) in BLACK.iter().enumerate() {
        result.push((14 + n, UPPER[i], 565, 72, 133));
    }
    for (i, n) in WHITE.iter().enumerate() {
        result.push((14 + n, 80 + i * 84, 706, 72, 130));
    }
    result
}
fn contains(x: f32, y: f32, r: (usize, usize, usize, usize)) -> bool {
    x >= r.0 as f32 && y >= r.1 as f32 && x < (r.0 + r.2) as f32 && y < (r.1 + r.3) as f32
}
pub fn hit(x: f32, y: f32) -> Option<panel::Ctl> {
    for (i, (cx, cy)) in ENCS.iter().enumerate() {
        if (x - cx).powi(2) + (y - cy).powi(2) <= 40_f32.powi(2) {
            return Some(panel::Ctl::Enc(i));
        }
    }
    if (x - 126.).powi(2) + (y - 155.).powi(2) <= 40_f32.powi(2) {
        return Some(panel::Ctl::Pot);
    }
    for (id, px, py, w, h) in buttons() {
        if contains(x, y, (px, py, w, h)) {
            return Some(panel::Ctl::Btn(id));
        }
    }
    for (id, px, py, w, h) in keys() {
        if contains(x, y, (px, py, w, h)) {
            return Some(panel::Ctl::Key(id));
        }
    }
    None
}
pub fn base(color: CaseColor) -> Vec<u32> {
    let bytes: &[u8] = match color {
        CaseColor::Black => include_bytes!("../resources/panel/black.png"),
        CaseColor::Green => include_bytes!("../resources/panel/green.png"),
        CaseColor::Orange => include_bytes!("../resources/panel/orange.png"),
        CaseColor::Purple => include_bytes!("../resources/panel/purple.png"),
        CaseColor::White => include_bytes!("../resources/panel/white.png"),
    };
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().expect("embedded panel PNG");
    let mut data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut data).expect("embedded panel pixels");
    assert_eq!((info.width, info.height), (W as u32, H as u32));
    let channels = info.color_type.samples();
    data[..info.buffer_size()]
        .chunks_exact(channels)
        .map(|p| (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32)
        .collect()
}
fn knob(buf: &mut [u32], base: &[u32], cx: f32, cy: f32, angle: f32, glow: bool) {
    let (sin, cos) = angle.sin_cos();
    for dy in -33..=33 {
        for dx in -33..=33 {
            let radius = (dx * dx + dy * dy) as f32;
            if radius > 33. * 33. {
                continue;
            }
            let sx = (cx + dx as f32 * cos + dy as f32 * sin).round() as usize;
            let sy = (cy - dx as f32 * sin + dy as f32 * cos).round() as usize;
            let at = (cy as i32 + dy) as usize * W + (cx as i32 + dx) as usize;
            buf[at] = if glow {
                panel::mix(base[sy * W + sx], 0xffffff, 0.05)
            } else {
                base[sy * W + sx]
            };
        }
    }
}
/// Shade the molded cap, leaving its surrounding tray and shadow stationary.
fn cap(
    buf: &mut [u32],
    base: &[u32],
    r: (usize, usize, usize, usize),
    down: bool,
    led: f32,
    tint: u32,
    pill: bool,
) {
    if !down && led <= 0. {
        return;
    }
    let (x, y, w, h) = r;
    let radius = if pill { (w - 14) as f32 / 2. } else { 8. };
    for py in 7..h - 8 {
        for px in 7..w - 7 {
            let qx = ((px as f32 - w as f32 / 2.).abs() - (w as f32 / 2. - 7. - radius)).max(0.);
            let qy = ((py as f32 - h as f32 / 2.).abs() - (h as f32 / 2. - 8. - radius)).max(0.);
            if qx * qx + qy * qy > radius * radius {
                continue;
            }
            let source = (y + py - if down { 2 } else { 0 }) * W + x + px;
            let v = if down {
                panel::mix(base[source], 0x000000, 0.14)
            } else {
                base[source]
            };
            buf[(y + py) * W + x + px] = panel::mix(v, tint, led * 0.24);
        }
    }
}
pub fn draw(buf: &mut [u32], base: &[u32], st: &panel::State, menu_down: bool, menu_hover: bool) {
    buf.copy_from_slice(base);
    // Firmware framebuffer replaces every illustrative screen pixel in the artwork.
    for y in 0..263 {
        for x in 0..282 {
            let v = st.lcd[y * 240 / 263 * 240 + x * 240 / 282] as u32;
            buf[(123 + y) * W + 424 + x] = ((v >> 11) * 255 / 31) << 16
                | (((v >> 5) & 63) * 255 / 63) << 8
                | (v & 31) * 255 / 31;
        }
    }
    for (i, (x, y)) in ENCS.iter().enumerate() {
        knob(
            buf,
            base,
            *x,
            *y,
            (st.enc_clicks[i] as f32 * 18.).to_radians(),
            st.turning[i] > 0,
        );
    }
    // Artwork's master pointer is at -15 degrees.
    knob(
        buf,
        base,
        126.,
        155.,
        (-120. + 270. * st.pot as f32 / 1023.).to_radians(),
        st.turning[7] > 0,
    );
    for (id, x, y, w, h) in buttons() {
        cap(
            buf,
            base,
            (x, y, w, h),
            st.down[id as usize],
            (st.leds[id as usize] / 0.08).clamp(0., 1.),
            if id == 12 { panel::GREEN } else { panel::AMBER },
            false,
        );
    }
    for (id, x, y, w, h) in keys() {
        cap(
            buf,
            base,
            (x, y, w, h),
            st.down[id as usize],
            (st.leds[id as usize] / 0.08).clamp(0., 1.),
            panel::AMBER,
            true,
        );
    }
    cap(
        buf,
        base,
        MENU,
        menu_down,
        if menu_hover { 0.35 } else { 0. },
        0xffffff,
        false,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn artwork_controls_keep_the_hardware_matrix_ids() {
        for (id, x, y, w, h) in keys() {
            assert_eq!(
                hit((x + w / 2) as f32, (y + h / 2) as f32),
                Some(panel::Ctl::Key(id))
            );
        }
        for (id, x, y, w, h) in buttons() {
            assert_eq!(
                hit((x + w / 2) as f32, (y + h / 2) as f32),
                Some(panel::Ctl::Btn(id))
            );
        }
        for (i, (x, y)) in ENCS.iter().enumerate() {
            assert_eq!(hit(*x, *y), Some(panel::Ctl::Enc(i)));
        }
        assert_eq!(hit(126., 155.), Some(panel::Ctl::Pot));
        assert_eq!(hit(560., 250.), None);
        assert_eq!(hit(1320., 510.), None);
        assert_eq!(
            keys()
                .iter()
                .map(|v| v.0)
                .collect::<std::collections::HashSet<_>>()
                .len(),
            27
        );
    }
    #[test]
    fn all_embedded_themes_render_live_lcd_and_key_feedback() {
        for color in CaseColor::ALL {
            let base = base(color);
            let mut buf = vec![0; W * H];
            let down = [false; 41];
            let lcd = vec![0xf800; 240 * 240];
            let mut state = panel::State {
                lcd: &lcd,
                enc_clicks: &[0; 7],
                pot: 512,
                down: &down,
                leds: &[0.; 41],
                turning: &[0; 8],
            };
            draw(&mut buf, &base, &state, false, false);
            assert_eq!(buf[200 * W + 500], 0xff0000);
            let released = buf[780 * W + 110];
            let mut pressed = down;
            pressed[14] = true;
            state.down = &pressed;
            draw(&mut buf, &base, &state, false, false);
            assert_ne!(buf[780 * W + 110], released);
            assert_eq!(base[200 * W + 500], 0, "no baked preview screen");
        }
    }
}
