//! Host positions drive relative hardware encoders, not firmware-specific sound values.
use nih_plug::prelude::*;
use serde::{Deserialize, Serialize};

pub const ENCODER_NAMES: [&str; 8] = [
    "Select",
    "Algorithm",
    "Knob 1",
    "Knob 2",
    "Knob 3",
    "Knob 4",
    "Presets",
    "SMK Knob 8",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binding {
    pub channel: u8,
    pub cc: u8,
}

/// Direct MIDI relative encoders bypass the host's bounded automation positions.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MidiEncoders {
    pub bindings: [Option<Binding>; 8],
    #[serde(skip)]
    pub learning: Option<usize>,
    #[serde(skip)]
    pub turns: [i32; 8],
}
impl MidiEncoders {
    pub fn route(&mut self, bytes: [u8; 3], len: u8) -> Option<(usize, i32)> {
        if len != 3 || bytes[0] & 0xf0 != 0xb0 || bytes[1] > 127 || bytes[2] > 127 {
            return None;
        }
        let binding = Binding {
            channel: bytes[0] & 15,
            cc: bytes[1],
        };
        if let Some(i) = self.learning.take().filter(|&i| i < 8) {
            for b in &mut self.bindings {
                if *b == Some(binding) {
                    *b = None;
                }
            }
            self.bindings[i] = Some(binding);
        }
        let i = self.bindings.iter().position(|b| *b == Some(binding))?;
        // Novation FLkey 2 Relative mode: binary offset, 63=-1 / 64=0 / 65=+1.
        let clicks = bytes[2] as i32 - 64;
        self.turns[i] = self.turns[i].wrapping_add(clicks);
        Some((i, clicks))
    }
}

#[derive(Params)]
pub struct Encoders {
    #[id = "encoder-select"]
    pub select: IntParam,
    #[id = "encoder-algorithm"]
    pub algorithm: IntParam,
    #[id = "encoder-knob1"]
    pub knob1: IntParam,
    #[id = "encoder-knob2"]
    pub knob2: IntParam,
    #[id = "encoder-knob3"]
    pub knob3: IntParam,
    #[id = "encoder-knob4"]
    pub knob4: IntParam,
    #[id = "encoder-presets"]
    pub presets: IntParam,
    #[id = "encoder-8"]
    pub eighth: IntParam,
}
impl Default for Encoders {
    fn default() -> Self {
        let p = |name| IntParam::new(name, 64, IntRange::Linear { min: 0, max: 127 });
        Self {
            select: p("Select"),
            algorithm: p("Algorithm"),
            knob1: p("Knob 1"),
            knob2: p("Knob 2"),
            knob3: p("Knob 3"),
            knob4: p("Knob 4"),
            presets: p("Presets"),
            eighth: p("SMK Knob 8"),
        }
    }
}
impl Encoders {
    pub fn get(&self, i: usize) -> &IntParam {
        [
            &self.select,
            &self.algorithm,
            &self.knob1,
            &self.knob2,
            &self.knob3,
            &self.knob4,
            &self.presets,
            &self.eighth,
        ][i]
    }
    pub fn positions(&self) -> [i32; 8] {
        std::array::from_fn(|i| self.get(i).value())
    }
}

pub fn deltas(previous: &mut [i32; 8], current: [i32; 8]) -> [i32; 8] {
    let changes = std::array::from_fn(|i| current[i] - previous[i]);
    *previous = current;
    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relative_messages_keep_moving_past_host_endpoints_and_reverse() {
        let mut map = MidiEncoders::default();
        map.learning = Some(6);
        for _ in 0..1000 {
            assert_eq!(map.route([0xb4, 21, 65], 3), Some((6, 1)));
        }
        assert_eq!(map.turns[6], 1000);
        for _ in 0..1200 {
            assert_eq!(map.route([0xb4, 21, 63], 3), Some((6, -1)));
        }
        assert_eq!(map.turns[6], -200);
        assert_eq!(map.route([0xb4, 21, 68], 3), Some((6, 4)));
        assert_eq!(map.route([0xb4, 21, 64], 3), Some((6, 0)));
        assert_eq!(map.route([0xb3, 21, 65], 3), None);
        assert_eq!(map.route([0xb4, 22, 65], 3), None);
        assert_eq!(map.route([0x94, 21, 65], 3), None);
    }
    #[test]
    fn learn_is_unique_and_bindings_recall_without_old_turns_or_learning() {
        let params = crate::Fm1Params::default();
        {
            let mut map = params.encoder_midi.lock().unwrap();
            map.learning = Some(2);
            map.route([0xb0, 22, 65], 3);
            map.learning = Some(6);
            map.route([0xb0, 22, 65], 3);
            assert_eq!(map.bindings[2], None);
            map.learning = Some(4);
        }
        let saved = params.serialize_fields();
        let restored = crate::Fm1Params::default();
        restored.deserialize_fields(&saved);
        let mut map = restored.encoder_midi.lock().unwrap();
        assert_eq!(map.learning, None);
        assert_eq!(map.turns, [0; 8]);
        assert_eq!(map.route([0xb0, 22, 63], 3), Some((6, -1)));
        let mut old = saved;
        old.remove("encoder-midi");
        let legacy = crate::Fm1Params::default();
        legacy.deserialize_fields(&old);
        assert_eq!(legacy.encoder_midi.lock().unwrap().bindings, [None; 8]);
    }
    #[test]
    fn positions_send_only_movement_and_rebasing_does_not_replay_it() {
        let mut previous = [64; 8];
        let mut current = previous;
        current[2] = 68;
        current[6] = 61;
        assert_eq!(deltas(&mut previous, current), [0, 0, 4, 0, 0, 0, -3, 0]);
        assert_eq!(deltas(&mut previous, current), [0; 8]);
        current[2] = 65;
        assert_eq!(deltas(&mut previous, current)[2], -3);
        previous = [100; 8]; // a restored project/firmware boot establishes a new baseline
        assert_eq!(deltas(&mut previous, [100; 8]), [0; 8]);
    }
}
