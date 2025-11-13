use super::voice::Voice;


pub trait NotesToUiVoices {
    fn to_ui_voices(&self) -> Vec<Voice>;
}

