// mochou-p/dondon/src/ui.rs

use std::sync::atomic::Ordering;
use std::ops::Range;
use cpal::platform::Stream;
use cpal::traits::StreamTrait;
use eframe::egui;
use eframe::Frame;
use egui::{Align2, FontId, Key, Painter, Rangef, ScrollArea, Stroke, StrokeKind, Panel, Rect, Sense, Ui, pos2, vec2};
use rtrb::Producer;
use crate::audio::{UiCommand, CLOCK};
use crate::theme::Theme;


struct State {
    audio_producer: Producer<UiCommand>,
    sample_rate:    f32,
    theme:          Theme,
    playing:        bool
}

impl State {
    fn new(audio_producer: Producer<UiCommand>, sample_rate: f32) -> Self {
        Self { audio_producer, sample_rate, theme: Theme::catppuccin_mocha(), playing: false }
    }

    fn callback(mut self) -> impl FnMut(&mut Ui, &mut Frame) {
        move |ui, _| {
            ui.ctx().input(|input| {
                if input.key_pressed(Key::Space) {
                    self.play_or_stop();
                }
            });

            if self.playing {
                ui.ctx().request_repaint();
            }

            self.menu(ui);
            self.piano_roll(ui);
        }
    }

    fn play_or_stop(&mut self) {
        self.audio_producer.push(
            if self.playing {
                UiCommand::Pause
            } else {
                UiCommand::Resume
            }
        ).unwrap();

        self.playing = !self.playing;
    }

    fn menu(&mut self, ui: &mut Ui) {
        Panel::top("menu")
            .show_inside(ui, |ui| {
                let text = if self.playing { "⏸" } else { "▶" };

                if ui.button(text).clicked() {
                    self.play_or_stop();
                }
            });
    }

    fn piano_roll(&self, ui: &mut Ui) {
        ScrollArea::both()
            .show(ui, |ui| {
                let screen_width      = ui.viewport_rect().width();
                let   line_height     = 20.0;
                let   line_width      = line_height * 200.0;
                let   full_height     = line_height * 88.0;
                let  white_height     = (line_height * 12.0) / 7.0;
                let  white_width      = white_height * 2.0;
                let  black_height     = line_height;
                let  black_width      = black_height * 2.0;

                let (rect, _response) = ui.allocate_exact_size(vec2(line_width, full_height), Sense::empty());
                let  painter          = ui.painter().with_clip_rect(rect);

                self.piano_roll_lines          (&painter, &rect, white_width, screen_width, line_height,     full_height);
                self.piano_roll_seek_bar       (&painter, &rect, white_width, full_height,  self.sample_rate            );
                self.piano_roll_white_key_block(&painter, &rect, white_width, full_height,  line_height                 );
                self.piano_roll_black_keys     (&painter, &rect, black_width, black_height, line_height                 );
            });
    }

    fn piano_roll_lines(
        &self,
        painter:      &Painter,
        rect:         &Rect,
        white_width:  f32,
        screen_width: f32,
        line_height:  f32,
        full_height:  f32
    ) {
        self._piano_roll_lines(
            painter,
            11..12,
            white_width,
            rect.min.y - line_height * 11.0,
            screen_width - white_width,
            line_height
        );

        let mut y_off = line_height;

        for _ in 0..7 {
            self._piano_roll_lines(
                painter,
                0..12,
                white_width,
                rect.min.y + y_off,
                screen_width - white_width,
                line_height
            );
            y_off += line_height * 12.0;
        }

        self._piano_roll_lines(
            painter,
            0..3,
            white_width,
            rect.min.y + y_off,
            screen_width - white_width,
            line_height
        );

        {
            let     width_per_second = 64.0;
            let mut x                = rect.min.x + white_width;
            let mut i                = 0;

            while x <= screen_width {
                painter.vline(
                    x,
                    Rangef::new(rect.min.y, rect.min.y + full_height), // NOTE: can be floating
                    Stroke::new(
                        0.25f32 + 0.75 * (i % 4 == 0) as i32 as f32 + (i % 16 == 0) as i32 as f32,
                        self.theme.piano_roll.outline
                    )
                );

                x += width_per_second * 0.25;
                i += 1;
            }
        }
    }

    fn _piano_roll_lines(
        &self,
        painter: &Painter,
        range:   Range<i32>,
        x:       f32,
        y:       f32,
        w:       f32,
        h:       f32
    ) {
        for i in range {
            let color = if i % 2 == (i > 6) as i32 {
                self.theme.piano_roll.white_key_line
            } else {
                self.theme.piano_roll.black_key_line
            };

            painter.rect(
                Rect::from_min_size(pos2(x, y + i as f32 * h), vec2(w, h)),
                0,
                color,
                Stroke::new(1.0f32, self.theme.piano_roll.outline),
                StrokeKind::Middle
            );
        }
    }

    fn piano_roll_seek_bar(
        &self,
        painter:     &Painter,
        rect:        &Rect,
        white_width: f32,
        full_height: f32,
        sample_rate: f32
    ) {
        let time             = CLOCK.load(Ordering::Relaxed) as f32 / sample_rate;
        let width_per_second = 64.0;
        let x                = time * width_per_second;

        painter.vline(
            rect.min.x + white_width + x,
            Rangef::new(rect.min.y, rect.min.y + full_height), // NOTE: can be floating
            Stroke::new(3.0f32, self.theme.piano_roll.seek_bar)
        );
    }

    fn piano_roll_white_key_block(
        &self,
        painter:     &Painter,
        rect:        &Rect,
        white_width: f32,
        full_height: f32,
        line_height: f32
    ) {
        painter.rect(
            Rect::from_min_size(pos2(0.0, 0.0), vec2(white_width, full_height)),
            0,
            self.theme.piano_roll.white_key,
            Stroke::new(1.0f32, self.theme.piano_roll.outline),
            StrokeKind::Outside
        );

        painter.text(
            pos2(white_width, rect.min.y),
            Align2::RIGHT_TOP,
            format!("C8"),
            FontId::monospace(line_height),
            self.theme.piano_roll.c_key_text
        );

        let mut y_off = 0.0;

        for i in 0..7 {
            painter.hline(
                Rangef::new(0.0, white_width),
                rect.min.y + line_height + y_off,
                Stroke::new(1.0f32, self.theme.piano_roll.key_separator)
            );
            painter.hline(
                Rangef::new(0.0, white_width),
                rect.min.y + line_height * 8.0 + y_off,
                Stroke::new(1.0f32, self.theme.piano_roll.key_separator)
            );

            y_off += line_height * 12.0;

            painter.text(
                pos2(white_width, rect.min.y + y_off),
                Align2::RIGHT_TOP,
                format!("C{}", 7 - i),
                FontId::monospace(line_height),
                self.theme.piano_roll.c_key_text
            );
        }

        painter.hline(
            Rangef::new(0.0, white_width),
            rect.min.y + line_height + y_off,
            Stroke::new(1.0f32, self.theme.piano_roll.key_separator)
        );
    }

    fn piano_roll_black_keys(
        &self,
        painter:      &Painter,
        rect:         &Rect,
        black_width:  f32,
        black_height: f32,
        line_height:  f32
    ) {
        for i in 0..7 {
            self._piano_roll_black_keys(
                painter,
                0..6,
                -black_height,
                rect.min.y + line_height + i as f32 * line_height * 12.0,
                black_width + black_height,
                black_height,
                line_height
            );
        }

        self._piano_roll_black_keys(
            painter,
            0..1,
            -black_height,
            rect.min.y + line_height + 7.0 * line_height * 12.0,
            black_width + black_height,
            black_height,
            line_height
        );
    }

    fn _piano_roll_black_keys(
        &self,
        painter:     &Painter,
        range:       Range<i32>,
        x:           f32,
        y:           f32,
        w:           f32,
        h:           f32,
        line_height: f32
    ) {
        for i in range {
            if i == 3 {
                continue;
            }

            let y_off = (i * 2 + (i < 3) as i32) as f32 * line_height;

            painter.rect_filled(
                Rect::from_min_size(pos2(x, y + y_off), vec2(w, h)),
                h * 0.15,
                self.theme.piano_roll.black_key
            );
        }
    }
}

pub fn run(stream: Stream, audio_producer: Producer<UiCommand>, sample_rate: f32) {
    let title   = std::env!("CARGO_BIN_NAME");
    let options = eframe::NativeOptions::default();

    let state   = State::new(audio_producer, sample_rate);

    stream.play().unwrap();
    eframe::run_ui_native(title, options, state.callback()).unwrap();
    stream.pause().unwrap();
}

