use nannou::prelude::*;

use super::palette::Palette;
use super::voice::Voice;


const PIXELS_PER_SECOND: f32 = 100.0;

pub struct PianoRoll {
    scroll: (f32, f32),
    seek:   f32
}

impl PianoRoll {
    pub const fn new() -> Self {
        let scroll = (0.0, 0.0);
        let seek   =  0.0;

        Self { scroll, seek }
    }

    pub fn event(&mut self, since_last: f32) {
        self.seek += since_last;
    }

    pub fn view(&self, app: &App, draw: &Draw, voices: &[Voice], palette: &Palette) {
        let win = app.window_rect();

        let row_wh     = Vec2::new(win.w(), 20.0);
        let key_w      = row_wh.y * 6.0;
        let row_colors = [
            palette.bg[1],
            palette.bg[0],
            palette.bg[1],
            palette.bg[0],
            palette.bg[1],
            palette.bg[1],
            palette.bg[0],
            palette.bg[1],
            palette.bg[0],
            palette.bg[1],
            palette.bg[0],
            palette.bg[1]
        ];

        for o in 0..5 {
            let mut row_xy = Vec2::new(0.0, win.h().mul_add(-0.5, row_wh.y * 0.5));

            row_xy.y += o as f32 * row_wh.y * 12.0;

            // rows
            for row_color in &row_colors {
                draw.rect()
                    .xy(row_xy)
                    .wh(row_wh)
                    .color(*row_color);

                row_xy.y += row_wh.y;
            }

            // row lines
            let mut start = pt2(win.w() * 0.5, row_wh.y.mul_add(-7.5, row_xy.y));
            let mut end   = pt2(-start.x, start.y);

            draw.line()
                .start(start)
                .end(end)
                .weight(1.0)
                .color(palette.bg[0]);

            start.y -= row_wh.y * 5.0;
            end  .y -= row_wh.y * 5.0;

            draw.line()
                .start(start)
                .end(end)
                .weight(2.0)
                .color(BLACK);

            let mut key_wh = Vec2::new(key_w, row_wh.y / 7.0 * 12.0);
            let mut key_xy = win.wh() * -0.5 + key_wh * 0.5;

            key_xy.y += o as f32 * row_wh.y * 12.0;

            // white keys
            for i in 0..7 {
                draw.rect()
                    .xy(key_xy)
                    .wh(key_wh)
                    .color(palette.fg[i % 2]);

                key_xy.y += key_wh.y;
            }

            // C label
            key_xy.x += key_w    * 0.33;
            key_xy.y -= key_wh.y * 6.95;

            draw.text(&format!("C{o}"))
                .xy(key_xy)
                .font_size(16)
                .color(palette.bg[0]);

            key_xy    = win.wh() * -0.5 + key_wh * 0.5;

            key_xy.y += o as f32 * row_wh.y * 12.0;

            key_xy.y += key_wh.y * 0.5;
            key_xy.x -= key_wh.x * 0.5;
            key_wh.y *= 0.5;

            // black keys
            for i in 0..5 {
                draw.rect()
                    .xy(key_xy)
                    .wh(key_wh)
                    .color(BLACK);

                let mul = if i == 1 { 2.0 } else { 1.0 };

                key_xy.y += key_wh.y * 2.0 * mul;
            }
        }

        for voice in voices {
            let     voice_len = 2.0;
            let mut voice_xy  = Vec2::new(win.w().mul_add(-0.5, key_w), win.h().mul_add(-0.5, row_wh.y * 0.5));
            let     voice_wh  = Vec2::new(voice_len * PIXELS_PER_SECOND, row_wh.y);

            voice_xy.x += voice_wh.x * 0.5;
            voice_xy.y += row_wh.y * <f32 as From<u8>>::from(voice.note.ui_row());

            draw.rect()
                .xy(voice_xy)
                .wh(voice_wh)
                .color(palette.bg[7]);
        }

        // seek bar
        let mut seek_start  = pt2(win.w().mul_add(-0.5, key_w), win.h() * 0.5);
        seek_start.x       += self.seek * PIXELS_PER_SECOND;
        let     seek_end    = pt2(seek_start.x, -seek_start.y);


        draw.line()
            .start(seek_start)
            .end(seek_end)
            .weight(4.0)
            .color(palette.bg[5]);
    }
}

