// mochou-p/dondon/src/scheduler.rs

use std::sync::atomic::Ordering;
use std::thread::{self, JoinHandle, sleep};
use std::time::Duration;
use rtrb::Producer;
use crate::RUNNING;
use crate::audio::{SchedulerCommand, CLOCK};


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
    sample_rate:    f32,
    notes:          Vec<Note>,
    i:              usize
}

impl State {
    fn new(audio_producer: Producer<SchedulerCommand>, sample_rate: f32) -> Self {
        let notes = vec![Note::new(440.0, 2.0, 6.0), Note::new(880.0, 4.0, 6.0)];

        Self { audio_producer, sample_rate, notes, i: 0 }
    }

    fn callback(mut self) -> impl FnMut() {
        move || {
            while RUNNING.load(Ordering::Relaxed) {
                let time = CLOCK.load(Ordering::Relaxed) as f32 / self.sample_rate;

                if self.i < self.notes.len() && time >= self.notes[self.i].start - 0.1 {
                    self.audio_producer.push(self.notes[self.i].to_command()).unwrap();
                    self.i += 1;
                }

                sleep(Duration::from_millis(5));
            }
        }
    }
}

pub fn spawn(audio_producer: Producer<SchedulerCommand>, sample_rate: f32) -> JoinHandle<()> {
    let state = State::new(audio_producer, sample_rate);

    thread::spawn(state.callback())
}

