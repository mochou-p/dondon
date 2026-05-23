// mochou-p/dondon/src/main.rs

mod audio;
mod theme;
mod ui;


struct State {
    _stream: cpal::Stream,
    ui:      ui::State
}

impl State {
    fn new() -> Self {
        let (producer, consumer   ) = rtrb::RingBuffer::new(8);
        let (_stream,  sample_rate) = audio::stream(consumer);
        let  theme                  = theme::Theme::catppuccin_mocha();
        let  ui                     = ui::State::new(producer, sample_rate, theme);

        Self { _stream, ui }
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        self.ui.ui(ui);
    }
}

enum Command {
    Resume,
    Pause
}

fn main() {
    let mut state = State::new();

    eframe::run_ui_native(
        std::env!("CARGO_BIN_NAME"),
        eframe::NativeOptions::default(),
        move |ui, _frame| state.ui(ui)
    ).unwrap();
}

