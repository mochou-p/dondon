mod palette;
mod theme;
mod traits;
mod voice;

use std::sync::{Arc, Mutex};

use nannou::prelude::*;

use super::audio::{self, Note};

use crate::audio::NotesToAudioVoices;

use theme::Theme;

pub use {traits::NotesToUiVoices, voice::Voice};


pub struct Model {
    ui_voices:    Vec<Voice>,
    audio_voices: Arc<Mutex<Vec<audio::Voice>>>,
    theme:        Theme
}

pub fn model(_app: &App) -> Model {
    let notes = vec![
        Note::D(3),
        Note::E(3),
        Note::A(3),
        Note::C(4)
    ];

    let    ui_voices = notes.to_ui_voices();
    let audio_voices = Arc::new(Mutex::new(notes.to_audio_voices()));
    let        theme = Theme::default();

    audio::spawn_thread(Arc::clone(&audio_voices));

    Model { ui_voices, audio_voices, theme }
}

pub fn event(_app: &App, _model: &mut Model, event: Event) {
    println!("\x1b[34;1mevent:\x1b[0m {event:?}");
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(model.theme.palette.bg[0]);

    draw.to_frame(app, &frame).unwrap();
}

