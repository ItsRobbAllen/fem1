use crate::panel::Palette;
use serde::{Deserialize, Serialize};

/// Hardware-inspired finishes from the linked Amazon listing. Cosmetic only.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaseColor {
    #[default]
    Black,
    Green,
    Orange,
    Purple,
    White,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{panel, Fm1Params};
    use nih_plug::prelude::Params;

    #[test]
    fn case_color_survives_project_state_and_old_projects_keep_the_default() {
        let params = Fm1Params::default();
        *params.case_color.lock().unwrap() = CaseColor::Orange;
        let saved = params.serialize_fields();
        let restored = Fm1Params::default();
        restored.deserialize_fields(&saved);
        assert_eq!(*restored.case_color.lock().unwrap(), CaseColor::Orange);
        let mut legacy = saved;
        legacy.remove("case-color");
        let restored = Fm1Params::default();
        restored.deserialize_fields(&legacy);
        assert_eq!(*restored.case_color.lock().unwrap(), CaseColor::Black);
    }

    #[test]
    fn finishes_preserve_lcd_pixels_and_pressed_key_feedback() {
        let lcd = vec![0xf800; 240 * 240];
        let mut down = [false; 41];
        let leds = [0.0; 41];
        let enc_clicks = [0; 7];
        let turning = [0; 8];
        for color in CaseColor::ALL {
            let palette = color.palette();
            let base = panel::base_with_palette(&palette);
            let mut buf = base.clone();
            down[14] = false;
            let state = panel::State { lcd: &lcd, enc_clicks: &enc_clicks, pot: 900,
                down: &down, leds: &leds, turning: &turning };
            panel::draw_with_palette(&mut buf, &base, &state, &palette);
            assert_eq!(buf[panel::LCD_Y * panel::W + panel::LCD_X], 0xff0000);
            let key_pixel = 735 * panel::W + 80;
            let released = buf[key_pixel];
            down[14] = true;
            let state = panel::State { lcd: &lcd, enc_clicks: &enc_clicks, pot: 900,
                down: &down, leds: &leds, turning: &turning };
            panel::draw_with_palette(&mut buf, &base, &state, &palette);
            assert_ne!(released, buf[key_pixel], "pressed feedback for {color:?}");
        }
    }
}

impl CaseColor {
    pub const ALL: [Self; 5] = [Self::Black, Self::Green, Self::Orange, Self::Purple, Self::White];
    pub fn name(self) -> &'static str {
        match self { Self::Black => "Black", Self::Green => "Green", Self::Orange => "Orange", Self::Purple => "Purple", Self::White => "White" }
    }
    pub fn palette(self) -> Palette {
        let (body_top, body_bot, silk, key_top, key_bot, key_line) = match self {
            Self::Black => (0x343940, 0x252a31, 0xd8d9dc, 0x69646a, 0x4d494f, 0xb6b0b6),
            Self::Green => (0x343940, 0x252a31, 0xd8e5db, 0xaed9b7, 0x82ad8d, 0x496952),
            Self::Orange => (0xf08c36, 0xcd6324, 0x332820, 0x514946, 0x36312f, 0xb7a69b),
            Self::Purple => (0xa797ce, 0x7c69a8, 0x282236, 0xd8c9ed, 0xb6a3d3, 0x5a4973),
            Self::White => (0xe5e8e8, 0xbfc7cc, 0x323e4c, 0x8aadd1, 0x6386af, 0x344b68),
        };
        Palette { body_top, body_bot, silk, key_top, key_bot, key_line,
            btn_top: key_top, btn_bot: key_bot, btn_ink: key_line }
    }
}
