use super::oscillator::Oscillator;
use super::note::Note;

use crate::State;


const DEFAULT_AMPLITUDE: f32 = 0.1;

pub struct Voice {
    oscillator: Oscillator,
    amplitude:  f32
}

impl From<Oscillator> for Voice {
    fn from(oscillator: Oscillator) -> Self {
        let amplitude = DEFAULT_AMPLITUDE;

        Self { oscillator, amplitude }
    }
}

impl From<Note> for Voice {
    fn from(note: Note) -> Self {
        let oscillator = Oscillator::from(note);

        Self::from(oscillator)
    }
}

impl Voice {
    #[expect(dead_code)]
    pub fn from_frequency(frequency: f32) -> Self {
        let oscillator = Oscillator::from_frequency(frequency);

        Self::from(oscillator)
    }

    #[inline]
    pub fn render_and_mix(&mut self, data: &mut [f32], state: &State) {
        for frame in data.chunks_mut(state.channels) {
            let raw_value    = self.oscillator.render(state.sample_rate);
            let sample       = raw_value * self.amplitude;

            for sample_in_frame in frame.iter_mut() {
                *sample_in_frame += sample;
            }
        }
    }
}

