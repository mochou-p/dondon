use std::f32::consts;

use super::State;
use super::oscillator::Oscillator;


const DEFAULT_AMPLITUDE: f32   = 0.1;
const DEFAULT_PAN:       f32   = 0.0;

const      LEFT_CHANNEL: usize = 0;
const     RIGHT_CHANNEL: usize = 1;

pub struct Voice {
    oscillator: Oscillator,
    amplitude:  f32,
    pan:        f32
}

impl From<Oscillator> for Voice {
    fn from(oscillator: Oscillator) -> Self {
        let amplitude = DEFAULT_AMPLITUDE;
        let pan       = DEFAULT_PAN;

        Self { oscillator, amplitude, pan }
    }
}

impl Voice {
    #[expect(dead_code)]
    pub fn from_frequency(frequency: f32) -> Self {
        let oscillator = Oscillator::from_frequency(frequency);

        Self::from(oscillator)
    }

    #[inline]
    fn pan_gains(&self) -> (f32, f32) {
        let constant_power_pan = self.pan.mul_add(0.5, 0.5) * 0.5 * consts::PI;

        let left  = constant_power_pan.cos() * self.amplitude;
        let right = constant_power_pan.sin() * self.amplitude;

        (left, right)
    }

    #[inline]
    pub fn render_and_mix(&mut self, data: &mut [f32], state: &State) {
        for frame in data.chunks_mut(state.channels) {
            let sample = self.oscillator.render(state.sample_rate);

            let (left_gain, right_gain) = self.pan_gains();

            frame[ LEFT_CHANNEL] += sample *  left_gain;
            frame[RIGHT_CHANNEL] += sample * right_gain;
        }
    }
}

