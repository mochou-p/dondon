pub enum Note {
    A4
}

impl Note {
    pub fn frequency(&self) -> f32 {
        match self {
            Self::A4 => 440.0
        }
    }
}

