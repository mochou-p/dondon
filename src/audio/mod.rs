mod note;
mod oscillator;
mod traits;
mod voice;
mod waveform;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use cpal::{Sample, SampleFormat, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub use {note::Note, traits::NotesToAudioVoices, voice::Voice};


pub fn spawn_thread(voices_mutex: Arc<Mutex<Vec<Voice>>>) {
    const ASSUMED_SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;

    let host      = cpal::default_host();
    let device    = host.default_output_device().unwrap();
    let supported = device.default_output_config().unwrap();

    let sample_format = supported.sample_format();

    if sample_format != ASSUMED_SAMPLE_FORMAT {
        eprintln!("\x1b[33;1mwarning:\x1b[0m your default sample format is {sample_format}, but this forces {ASSUMED_SAMPLE_FORMAT}");
    }

    let config = supported.config();
    let state  = State::from(&config);

    thread::spawn(move || {
        let stream = device.build_output_stream(
            &config,
            move |data, _| {
                for sample in data.iter_mut() {
                    *sample = f32::EQUILIBRIUM;
                }

                {
                    let mut voices = voices_mutex.lock().unwrap();

                    for voice in voices.iter_mut() {
                        voice.render_and_mix(data, &state);
                    }
                }
            },
            |error| eprintln!("\x1b[31;1merror:\x1b[0m {error}"),
            None
        ).unwrap();

        stream.play().unwrap();
        thread::sleep(Duration::from_secs(2));
        stream.pause().unwrap();
    });
}

pub struct State {
    channels:    usize,
    sample_rate: f32
}

impl From<&StreamConfig> for State {
    fn from(config: &StreamConfig) -> Self {
        const ASSUMED_CHANNEL_COUNT: usize = 2;

        let channels    = config.channels      as usize;
        let sample_rate = config.sample_rate.0 as f32;

        if channels != ASSUMED_CHANNEL_COUNT {
            eprintln!("\x1b[33;1mwarning:\x1b[0m your default channel count is {channels}, but this has only been tested on {ASSUMED_CHANNEL_COUNT} channels");
        }

        Self { channels, sample_rate }
    }
}

