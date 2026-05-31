// mochou-p/dondon/src/ui.rs

use std::sync::atomic::Ordering;
use std::ops::Range;
use cpal::platform::Stream;
use cpal::traits::StreamTrait;
use eframe::egui;
use eframe::NativeOptions;
use egui::{Align2, CentralPanel, FontId, Frame, Key, Painter, PointerButton, Rangef, Response, ScrollArea, Stroke, StrokeKind, Panel, Rect, Sense, Ui, pos2, vec2};
use egui::containers::scroll_area::{ScrollSource, ScrollBarVisibility};
use rtrb::Producer;
use crate::audio::{self, CLOCK};
use crate::scheduler;
use crate::theme::Theme;


const WIDTH_PER_SECOND: f32 = 64.0;
const DEFAULT_LENGTH:   f32 =  1.0;

struct Note {
    start:  f32,
    length: f32,
    key:    f32
}

impl Note {
    fn new(start: f32, key: f32) -> Self {
        let step  = 0.25;
        let start = (start / step).round() * step;

        Self { start, length: DEFAULT_LENGTH, key }
    }

    fn to_command(&self) -> scheduler::UiCommand {
        let frequency = 440.0 * 2.0f32.powf(((88.0 - self.key) - 49.0) / 12.0);
        scheduler::UiCommand::NewNote { frequency, start: self.start, end: self.start + DEFAULT_LENGTH }
    }
}

struct State {
    audio_producer:     Producer<    audio::UiCommand>,
    scheduler_producer: Producer<scheduler::UiCommand>,
    sample_rate:        f32,
    theme:              Theme,
    playing:            bool,
    notes:              Vec<Note>,
    dragged_note:       Option<usize>
}

impl State {
    fn new(
        audio_producer:     Producer<    audio::UiCommand>,
        scheduler_producer: Producer<scheduler::UiCommand>,
        sample_rate:        f32
    ) -> Self {
        let theme   = Theme::catppuccin_mocha();
        let playing = false;

        Self { audio_producer, scheduler_producer, sample_rate, theme, playing, notes: vec![], dragged_note: None }
    }

    fn callback(mut self) -> impl FnMut(&mut Ui, &mut eframe::Frame) {
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
                audio::UiCommand::Pause
            } else {
                audio::UiCommand::Resume
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

    fn piano_roll(&mut self, ui: &mut Ui) {
        CentralPanel::default()
            .frame(Frame::default())
            .show_inside(ui, |ui| {
                ScrollArea::both()
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .scroll_source(ScrollSource::MOUSE_WHEEL)
                    .show(ui, |ui| {
                        let screen_width  = ui.viewport_rect().width();
                        let   line_height = 20.0;
                        let   line_width  = line_height * 200.0;
                        let   full_height = line_height * 88.0;
                        let  white_height = (line_height * 12.0) / 7.0;
                        let  white_width  = white_height * 2.0;
                        let  black_height = line_height;
                        let  black_width  = black_height * 2.0;

                        let (rect, response) = ui.allocate_exact_size(vec2(line_width, full_height), Sense::click_and_drag());
                        let  painter         = ui.painter().with_clip_rect(rect);

                        self.piano_roll_input(
                            &rect, &response, white_width, line_height
                        );
                        self.piano_roll_draw(
                            &painter, &rect, white_width, black_width, black_height, screen_width, full_height, line_height
                        );
                    });
            });
    }

    fn get_note_index(&self, finder: impl Fn(&Note) -> bool) -> Option<usize> {
        for (i, note) in self.notes.iter().enumerate().rev() {
            if finder(note) {
                return Some(i);
            }
        }

        None
    }

    fn get_note_indices(&self, finder: impl Fn(&Note) -> bool) -> Vec<usize> {
        let mut indices = vec![];

        for (i, note) in self.notes.iter().enumerate().rev() {
            if finder(note) {
                indices.push(i);
            }
        }

        indices
    }

    fn hovered_note(x: f32, key: f32) -> impl Fn(&Note) -> bool {
        move |note| x >= note.start && x < note.start + note.length && key.floor() == note.key
    }

    fn piano_roll_input(
        &mut self,
        rect:        &Rect,
        response:    &Response,
        white_width: f32,
        line_height: f32
    ) {
        let Some((x, y)) = response
            .interact_pointer_pos()
            .map(| pos  | (pos.x - rect.min.x - white_width, pos.y - rect.min.y))
            .map(|(x, y)| (x / WIDTH_PER_SECOND, y / line_height))
        else {
            return;
        };

        if self.piano_roll_input_click_primary         (response, x, y) { return; }
        if self.piano_roll_input_click_secondary       (response, x, y) { return; }
        if self.piano_roll_input_drag_started_primary  (response, x, y) { return; }
        if self.piano_roll_input_drag_started_secondary(response, x, y) { return; }
        if self.piano_roll_input_dragged_primary       (response, x, y) { return; }
        if self.piano_roll_input_dragged_secondary     (response, x, y) { return; }
        if self.piano_roll_input_drag_stopped_primary  (response      ) { return; }
    }

    fn piano_roll_input_click_primary(&mut self, response: &Response, x: f32, y: f32) -> bool {
        if response.clicked_by(PointerButton::Primary) {
            if x < 0.0 || y < 0.0 || y >= 88.0 { return true; }

            let note = Note::new(x, y.floor());

            self.scheduler_producer.push(note.to_command()).unwrap();
            self.notes.push(note);
            return true;
        }

        false
    }

    fn piano_roll_input_click_secondary(&mut self, response: &Response, x: f32, y: f32) -> bool {
        if response.clicked_by(PointerButton::Secondary) {
            if x < 0.0 || y < 0.0 || y >= 88.0 { return true; }

            let indices = self.get_note_indices(Self::hovered_note(x, y));

            for i in indices.iter() {
                self.notes.remove(*i); // TODO: tell scheduler (remove)
            }

            return true;
        }

        false
    }

    fn piano_roll_input_drag_started_primary(&mut self, response: &Response, x: f32, y: f32) -> bool {
        if response.drag_started_by(PointerButton::Primary) {
            if let Some(i) = self.get_note_index(Self::hovered_note(x, y)) {
                // TODO: tell scheduler (remove)
                self.dragged_note = Some(i);
                return true;
            }

            let i = self.notes.len();

            self.notes.push(Note::new(0.0, (y - 0.5).clamp(0.0, 87.0)));
            // TODO: temp temp temp temp
            self.notes[i].start = (x - DEFAULT_LENGTH * 0.5).max(0.0);

            self.dragged_note = Some(i);
            return true;
        }

        false
    }

    fn piano_roll_input_drag_started_secondary(&mut self, response: &Response, x: f32, y: f32) -> bool {
        if response.drag_started_by(PointerButton::Secondary) {
            let indices = self.get_note_indices(Self::hovered_note(x, y));

            for i in indices.iter() {
                self.notes.remove(*i); // TODO: tell scheduler (remove)
            }

            return true;
        }

        false
    }

    fn piano_roll_input_dragged_primary(&mut self, response: &Response, x: f32, y: f32) -> bool {
        if response.dragged_by(PointerButton::Primary) {
            if let Some(i) = self.dragged_note {
                self.notes[i].start = (x - self.notes[i].length * 0.5).max(0.0);
                self.notes[i].key   = (y - 0.5).clamp(0.0, 87.0);
            }
        }

        false
    }

    fn piano_roll_input_dragged_secondary(&mut self, response: &Response, x: f32, y: f32) -> bool {
        if response.dragged_by(PointerButton::Secondary) {
            let indices = self.get_note_indices(Self::hovered_note(x, y));

            for i in indices.iter() {
                self.notes.remove(*i); // TODO: tell scheduler (remove)
            }

            return true;
        }

        false
    }

    fn piano_roll_input_drag_stopped_primary(&mut self, response: &Response) -> bool {
        if response.drag_stopped_by(PointerButton::Primary) {
            if let Some(i) = self.dragged_note.take() {
                let step  = 0.25;
                let start = (self.notes[i].start / step).round() * step;
                let key   = self.notes[i].key.round();

                self.notes[i].start = start;
                self.notes[i].key   = key;

                // TODO: tell scheduler (new)
            }

            return true;
        }

        false
    }

    fn piano_roll_draw(
        &self,
        painter:      &Painter,
        rect:         &Rect,
        white_width:  f32,
        black_width:  f32,
        black_height: f32,
        screen_width: f32,
        full_height:  f32,
        line_height:  f32
    ) {
        self.piano_roll_draw_lines          (&painter, &rect, white_width, screen_width, line_height,      full_height);
        self.piano_roll_draw_notes          (&painter, &rect, white_width,               line_height                  );
        self.piano_roll_draw_seek_bar       (&painter, &rect, white_width, full_height,  self.sample_rate             );
        self.piano_roll_draw_white_key_block(&painter, &rect, white_width, full_height,  line_height                  );
        self.piano_roll_draw_black_keys     (&painter, &rect, black_width, black_height, line_height                  );
    }

    fn piano_roll_draw_lines(
        &self,
        painter:      &Painter,
        rect:         &Rect,
        white_width:  f32,
        screen_width: f32,
        line_height:  f32,
        full_height:  f32
    ) {
        self._piano_roll_draw_lines(
            painter,
            11..12,
            white_width,
            rect.min.y - line_height * 11.0,
            screen_width - white_width,
            line_height
        );

        let mut y_off = line_height;

        for _ in 0..7 {
            self._piano_roll_draw_lines(
                painter,
                0..12,
                white_width,
                rect.min.y + y_off,
                screen_width - white_width,
                line_height
            );
            y_off += line_height * 12.0;
        }

        self._piano_roll_draw_lines(
            painter,
            0..3,
            white_width,
            rect.min.y + y_off,
            screen_width - white_width,
            line_height
        );

        {
            let mut x = rect.min.x + white_width;
            let mut i = 0;

            while x <= screen_width {
                painter.vline(
                    x,
                    Rangef::new(rect.min.y, rect.min.y + full_height), // NOTE: can be floating
                    Stroke::new(
                        0.25f32 + 0.75 * (i % 4 == 0) as i32 as f32 + (i % 16 == 0) as i32 as f32,
                        self.theme.piano_roll.outline
                    )
                );

                x += WIDTH_PER_SECOND * 0.25;
                i += 1;
            }
        }
    }

    fn _piano_roll_draw_lines(
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

    fn piano_roll_draw_notes(
        &self,
        painter:     &Painter,
        rect:        &Rect,
        white_width: f32,
        line_height: f32
    ) {
        for note in self.notes.iter() {
            let x = rect.min.x + white_width + note.start * WIDTH_PER_SECOND;
            let y = rect.min.y + line_height * note.key;
            let w = note.length * WIDTH_PER_SECOND;

            painter.rect(
                Rect::from_min_size(pos2(x, y), vec2(w, line_height)),
                line_height * 0.25,
                self.theme.piano_roll.note,
                Stroke::new(1.0f32, self.theme.piano_roll.outline),
                StrokeKind::Inside
            );
        }
    }

    fn piano_roll_draw_seek_bar(
        &self,
        painter:     &Painter,
        rect:        &Rect,
        white_width: f32,
        full_height: f32,
        sample_rate: f32
    ) {
        let time = CLOCK.load(Ordering::Relaxed) as f32 / sample_rate;
        let x    = time * WIDTH_PER_SECOND;

        painter.vline(
            rect.min.x + white_width + x,
            Rangef::new(rect.min.y, rect.min.y + full_height), // NOTE: can be floating
            Stroke::new(3.0f32, self.theme.piano_roll.seek_bar)
        );
    }

    fn piano_roll_draw_white_key_block(
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

    fn piano_roll_draw_black_keys(
        &self,
        painter:      &Painter,
        rect:         &Rect,
        black_width:  f32,
        black_height: f32,
        line_height:  f32
    ) {
        for i in 0..7 {
            self._piano_roll_draw_black_keys(
                painter,
                0..6,
                -black_height,
                rect.min.y + line_height + i as f32 * line_height * 12.0,
                black_width + black_height,
                black_height,
                line_height
            );
        }

        self._piano_roll_draw_black_keys(
            painter,
            0..1,
            -black_height,
            rect.min.y + line_height + 7.0 * line_height * 12.0,
            black_width + black_height,
            black_height,
            line_height
        );
    }

    fn _piano_roll_draw_black_keys(
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

pub fn run(
    stream:             Stream,
    audio_producer:     Producer<    audio::UiCommand>,
    scheduler_producer: Producer<scheduler::UiCommand>,
    sample_rate:        f32
) {
    let title   = std::env!("CARGO_BIN_NAME");
    let options = NativeOptions::default();

    let state   = State::new(audio_producer, scheduler_producer, sample_rate);

    stream.play().unwrap();
    eframe::run_ui_native(title, options, state.callback()).unwrap();
    stream.pause().unwrap();
}

