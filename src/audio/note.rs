use super::oscillator::Oscillator;
use super::traits::NotesToAudioVoices;
use super::voice::Voice;

use crate::ui::{self, NotesToUiVoices};


#[expect(dead_code)]
pub enum Note {
    C (u8),
    Cs(u8),
    Db(u8),
    D (u8),
    Ds(u8),
    Eb(u8),
    E (u8),
    F (u8),
    Fs(u8),
    Gb(u8),
    G (u8),
    Gs(u8),
    Ab(u8),
    A (u8),
    As(u8),
    Bb(u8),
    B (u8)
}

impl NotesToUiVoices for Vec<Note> {
    fn to_ui_voices(self) -> Vec<ui::Voice> {
        self.into_iter()
            .map(ui::Voice::from)
            .collect()
    }
}

impl NotesToAudioVoices for Vec<Note> {
    fn to_audio_voices(&self) -> Vec<Voice> {
        self.iter()
            .map(Oscillator::from)
            .map(Voice::from)
            .collect()
    }
}

impl Note {
    const fn nth_in_octave(&self) -> u8 {
        match self {
            Self::C (_)               => 0,
            Self::Cs(_) | Self::Db(_) => 1,
            Self::D (_)               => 2,
            Self::Ds(_) | Self::Eb(_) => 3,
            Self::E (_)               => 4,
            Self::F (_)               => 5,
            Self::Fs(_) | Self::Gb(_) => 6,
            Self::G (_)               => 7,
            Self::Gs(_) | Self::Ab(_) => 8,
            Self::A (_)               => 9,
            Self::As(_) | Self::Bb(_) => 10,
            Self::B (_)               => 11
        }
    }

    const fn octave_zero_frequency(&self) -> f32 {
        match self {
            Self::C (_)               => 16.35160,
            Self::Cs(_) | Self::Db(_) => 17.32391,
            Self::D (_)               => 18.35405,
            Self::Ds(_) | Self::Eb(_) => 19.44544,
            Self::E (_)               => 20.60172,
            Self::F (_)               => 21.82676,
            Self::Fs(_) | Self::Gb(_) => 23.12465,
            Self::G (_)               => 24.49971,
            Self::Gs(_) | Self::Ab(_) => 25.95654,
            Self::A (_)               => 27.50000,
            Self::As(_) | Self::Bb(_) => 29.13524,
            Self::B (_)               => 30.86771
        }
    }

    const fn octave_number(&self) -> u8 {
        match self {
            Self::C (octave) |
            Self::Cs(octave) |
            Self::Db(octave) |
            Self::D (octave) |
            Self::Ds(octave) |
            Self::Eb(octave) |
            Self::E (octave) |
            Self::F (octave) |
            Self::Fs(octave) |
            Self::Gb(octave) |
            Self::G (octave) |
            Self::Gs(octave) |
            Self::Ab(octave) |
            Self::A (octave) |
            Self::As(octave) |
            Self::Bb(octave) |
            Self::B (octave) => *octave
        }
    }

    pub fn frequency(&self) -> f32 {
        self.octave_zero_frequency() * f32::from(self.octave_number()).exp2()
    }

    pub const fn ui_row(&self) -> u8 {
        self.nth_in_octave() + 12 * self.octave_number()
    }
}

