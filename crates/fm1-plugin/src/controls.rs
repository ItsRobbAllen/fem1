//! Host positions drive relative hardware encoders, not firmware-specific sound values.
use nih_plug::prelude::*;

#[derive(Params)]
pub struct Encoders {
    #[id = "encoder-select"] pub select: IntParam,
    #[id = "encoder-algorithm"] pub algorithm: IntParam,
    #[id = "encoder-knob1"] pub knob1: IntParam,
    #[id = "encoder-knob2"] pub knob2: IntParam,
    #[id = "encoder-knob3"] pub knob3: IntParam,
    #[id = "encoder-knob4"] pub knob4: IntParam,
    #[id = "encoder-presets"] pub presets: IntParam,
    #[id = "encoder-8"] pub eighth: IntParam,
}
impl Default for Encoders {
    fn default() -> Self {
        let p = |name| IntParam::new(name, 64, IntRange::Linear { min: 0, max: 127 });
        Self { select: p("Select"), algorithm: p("Algorithm"), knob1: p("Knob 1"),
            knob2: p("Knob 2"), knob3: p("Knob 3"), knob4: p("Knob 4"),
            presets: p("Presets"), eighth: p("SMK Knob 8") }
    }
}
impl Encoders {
    pub fn get(&self, i: usize) -> &IntParam {
        [&self.select, &self.algorithm, &self.knob1, &self.knob2,
            &self.knob3, &self.knob4, &self.presets, &self.eighth][i]
    }
    pub fn positions(&self) -> [i32; 8] { std::array::from_fn(|i| self.get(i).value()) }
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
