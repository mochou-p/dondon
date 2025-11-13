use super::note::Note;
use super::waveform::Waveform;


pub struct Oscillator {
    shape:     Waveform,
    phase:     f32,
    frequency: f32
}

impl From<&Note> for Oscillator {
    fn from(note: &Note) -> Self {
        let shape     = Waveform::Sine;
        let phase     = 0.0;
        let frequency = note.frequency();

        Self { shape, phase, frequency }
    }
}

impl Oscillator {
    pub fn from_frequency(frequency: f32) -> Self {
        let shape = Waveform::Sine;
        let phase = 0.0;

        Self { shape, phase, frequency }
    }

    #[inline]
    pub fn render(&mut self, sample_rate: f32) -> f32 {
        let sample = self.shape.sample(self.phase);

        let step   = self.frequency / sample_rate;
        self.phase = (self.phase + step).fract();

        sample
    }
}

