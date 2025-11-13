use super::palette::Palette;


pub struct Theme {
    pub palette: Palette
}

impl Default for Theme {
    fn default() -> Self {
        let palette = Palette::dark();

        Self { palette }
    }
}

