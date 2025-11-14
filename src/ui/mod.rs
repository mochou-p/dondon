mod palette;
mod piano_roll;
mod theme;
mod traits;
mod voice;

use std::sync::{Arc, Mutex};

use nannou::prelude::*;

use super::audio::{self, Note};

use crate::audio::NotesToAudioVoices;

use piano_roll::PianoRoll;
use theme::Theme;

pub use {traits::NotesToUiVoices, voice::Voice};


pub struct Model {
    audio_voices: Arc<Mutex<Vec<audio::Voice>>>,
    ui_voices:    Vec<Voice>,
    theme:        Theme,
    piano_roll:   PianoRoll
}

pub fn model(_app: &App) -> Model {
    let notes = vec![
        Note::D(3),
        Note::E(3),
        Note::A(3),
        Note::C(4)
    ];

    let audio_voices = Arc::new(Mutex::new(notes.to_audio_voices()));
    let    ui_voices = notes.to_ui_voices();
    let        theme = Theme::default();
    let   piano_roll = PianoRoll::new();

    audio::spawn_thread(Arc::clone(&audio_voices));

    Model { audio_voices, ui_voices, theme, piano_roll }
}

pub fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::Update(update) = event {
        model.piano_roll.event(update.since_last.as_secs_f32());
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.piano_roll.view(app, &draw, &model.ui_voices, &model.theme.palette);

    draw.to_frame(app, &frame).unwrap();
}

