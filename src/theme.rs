// mochou-p/dondon/src/theme.rs

use eframe::egui;
use egui::Color32;


pub struct Theme {
    pub piano_roll: PianoRollTheme
}

pub struct PianoRollTheme {
    pub white_key_line: Color32,
    pub black_key_line: Color32,
    pub key_separator:  Color32,
    pub outline:        Color32,
    pub white_key:      Color32,
    pub black_key:      Color32,
    pub seek_bar:       Color32,
    pub c_key_text:     Color32
}

impl Theme {
    // https://catppuccin.com/palette/
    pub fn catppuccin_mocha() -> Self {
        Self {
            piano_roll: PianoRollTheme {
                white_key_line: Color32::from_rgb( 49,  50,  68),
                black_key_line: Color32::from_rgb( 30,  30,  46),
                key_separator:  Color32::from_rgb(186, 194, 222),
                outline:        Color32::from_rgb( 17,  17,  27),
                white_key:      Color32::from_rgb(205, 214, 244),
                black_key:      Color32::from_rgb( 24,  24,  37),
                seek_bar:       Color32::from_rgb(137, 180, 250),
                c_key_text:     Color32::from_rgb( 88,  91, 112)
            }
        }
    }
}

