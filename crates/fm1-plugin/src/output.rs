//! Stereo-linked peak protection after the plug-in's output trim.
pub struct PeakGuard {
    gain: f32,
    release: f32,
}
impl Default for PeakGuard {
    fn default() -> Self {
        let mut guard = Self {
            gain: 1.0,
            release: 0.0,
        };
        guard.reset(44_100.0);
        guard
    }
}
impl PeakGuard {
    pub const CEILING: f32 = 0.98;
    pub fn reset(&mut self, sample_rate: f32) {
        self.gain = 1.0;
        self.release = (-1.0 / (sample_rate.max(1.0) * 0.060)).exp();
    }
    pub fn process(&mut self, input: [f32; 2]) -> [f32; 2] {
        let peak = input[0].abs().max(input[1].abs());
        let wanted = if peak > Self::CEILING {
            Self::CEILING / peak
        } else {
            1.0
        };
        // Instant attack, 60 ms recovery. A single gain preserves stereo balance.
        self.gain = (1.0 - (1.0 - self.gain) * self.release).min(wanted);
        [input[0] * self.gain, input[1] * self.gain]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quiet_signals_remain_linear_and_loud_peaks_keep_stereo_balance() {
        let mut guard = PeakGuard::default();
        let boost = nih_plug::util::db_to_gain(12.0);
        let input = [0.1 * boost, -0.05 * boost];
        assert_eq!(guard.process(input), input);
        let limited = guard.process([4.0, -2.0]);
        assert!((limited[0] - PeakGuard::CEILING).abs() < 1e-6);
        assert!((limited[1] + PeakGuard::CEILING / 2.0).abs() < 1e-6);
        for _ in 0..1000 {
            let v = guard.process([-8.0, 0.3]);
            assert!(v.iter().all(|s| s.abs() <= PeakGuard::CEILING + 1e-6));
        }
    }
    #[test]
    fn recovery_is_smooth_at_each_host_rate_and_reset_clears_reduction() {
        for rate in [44_100.0, 48_000.0, 96_000.0] {
            let mut guard = PeakGuard::default();
            guard.reset(rate);
            guard.process([4.0, 4.0]);
            let mut before = 0.0;
            for _ in 0..(rate * 0.360) as usize {
                let now = guard.process([0.1, 0.1])[0];
                assert!(now >= before && now - before < 0.025);
                before = now;
            }
            assert!(before > 0.099);
            guard.reset(rate);
            assert_eq!(guard.process([0.1, 0.1]), [0.1, 0.1]);
        }
    }
}
