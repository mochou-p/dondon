// mochou-p/dondon/src/ui.rs

use eframe::egui;
use egui::Ui;


pub struct State;

impl State {
    pub fn ui(&self, ui: &mut Ui) {
        ui.heading("hello :D");
    }
}

