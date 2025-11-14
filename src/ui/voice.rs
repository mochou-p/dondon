use crate::audio::Note;


pub struct Voice {
    pub note: Note
}

impl From<Note> for Voice {
    fn from(note: Note) -> Self {
        Self { note }
    }
}

