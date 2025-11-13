use super::oscillator::Oscillator;


pub struct Voice {
    oscillator: Oscillator,
    amplitude:  f32
}

impl From<Oscillator> for Voice {
    fn from(oscillator: Oscillator) -> Self {
        let amplitude = 1.0;

        Self { oscillator, amplitude }
    }
}

impl Voice {
    #[inline]
    pub fn render(&mut self, sample_rate: f32) -> f32 {
        let sample = self.oscillator.render(sample_rate);

        sample * self.amplitude
    }
}

