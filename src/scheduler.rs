// mochou-p/dondon/src/scheduler.rs

use std::sync::atomic::Ordering;
use std::thread::{self, JoinHandle, sleep};
use std::time::Duration;
use rtrb::{RingBuffer, Consumer, Producer};
use crate::RUNNING;
use crate::audio::{SchedulerCommand, CLOCK};


pub enum UiCommand {
    NewNote { frequency: f32, start: f32, end: f32 }
}

struct Note {
    start:     f32,
    end:       f32,
    amplitude: f32,
    frequency: f32
}

impl Note {
    fn new(frequency: f32, start: f32, end: f32) -> Self {
        Self { start, end, amplitude: 0.5, frequency }
    }

    fn to_command(&self) -> SchedulerCommand {
        SchedulerCommand::PlayNote { start: self.start, end: self.end, amplitude: self.amplitude, frequency: self.frequency }
    }
}

struct State {
    audio_producer: Producer<SchedulerCommand>,
    ui_consumer:    Consumer<       UiCommand>,
    sample_rate:    f32,
    notes:          Vec<Note>,
    i:              usize
}

impl State {
    fn new(audio_producer: Producer<SchedulerCommand>, ui_consumer: Consumer<UiCommand>, sample_rate: f32) -> Self {
        Self { audio_producer, ui_consumer, sample_rate, notes: vec![], i: 0 }
    }

    fn callback(mut self) -> impl FnMut() {
        move || {
            while RUNNING.load(Ordering::Relaxed) {
                let time = CLOCK.load(Ordering::Relaxed) as f32 / self.sample_rate;

                self.process_ui_commands(time);

                if self.i < self.notes.len() && time >= self.notes[self.i].start - 0.1 {
                    self.audio_producer.push(self.notes[self.i].to_command()).unwrap();
                    self.i += 1;
                }

                sleep(Duration::from_millis(5));
            }
        }
    }

    fn process_ui_commands(&mut self, time: f32) {
        while let Ok(command) = self.ui_consumer.pop() {
            match command {
                UiCommand::NewNote { frequency, start, end } => {
                    let note = Note::new(frequency, start, end);

                    if time > start && time < end {
                        self.audio_producer.push(note.to_command()).unwrap();
                    }

                    self.notes.push(note);

                    // TODO: this is bad, use a specialized collection
                    self.notes.sort_by(|a, b| a.start.total_cmp(&b.start));
                }
            }
        }
    }
}

pub fn spawn(audio_producer: Producer<SchedulerCommand>, sample_rate: f32) -> (JoinHandle<()>, Producer<UiCommand>) {
    let (ui_producer, ui_consumer) = RingBuffer::new(8);
    let  state                     = State::new(audio_producer, ui_consumer, sample_rate);

    (thread::spawn(state.callback()), ui_producer)
}

