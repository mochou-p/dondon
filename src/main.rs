// mochou-p/dondon/src/main.rs

mod audio;
mod ui;


struct State {
    stream: cpal::Stream,
    ui:     ui::State
}

impl State {
    fn new() -> Self {
        let stream = audio::stream();
        let ui     = ui::State;

        Self { stream, ui }
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        self.ui.ui(ui);
    }
}

fn main() {
    let mut state = State::new();

    eframe::run_ui_native(
        std::env!("CARGO_BIN_NAME"),
        eframe::NativeOptions::default(),
        move |ui, _frame| state.ui(ui)
    ).unwrap();
}

