use std::f32::consts;


#[expect(dead_code)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth
}

impl Waveform {
    #[inline]
    pub fn sample(&self, phase: f32) -> f32 {
        match self {
            Self::Sine => {
                let radians = phase * 2.0 * consts::PI;

                radians.sin()
            },
            Self::Square => {
                phase.round().mul_add(2.0, -1.0)
            },
            Self::Sawtooth => {
                phase.mul_add(2.0, -1.0)
            }
        }
    }
}

