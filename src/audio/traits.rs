use super::voice::Voice;


pub trait NotesToAudioVoices {
    fn to_audio_voices(&self) -> Vec<Voice>;
}

