mod note;
mod oscillator;
mod voice;
mod waveform;

use std::thread;
use std::time::Duration;

use cpal::{Device, Sample, StreamConfig, StreamError, SupportedStreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use note::Note;
use oscillator::Oscillator;
use voice::Voice;


fn main() {
    App::new().run();
}

struct App {
    device:    Device,
    supported: SupportedStreamConfig
}

impl App {
    fn new() -> Self {
        let host      = cpal::default_host();
        let device    = host.default_output_device().unwrap();
        let supported = device.default_output_config().unwrap();

        Self { device, supported }
    }

    fn run(&mut self) {
        let mut voice  = Voice::from(Oscillator::from(Note::A4));
        let     config = self.supported.config();
        let     state  = State::from(&config);

        let stream = self.device.build_output_stream(
            &config,
            move |data: &mut [i16], _| {
                Self::data_callback(data, &state, &mut voice);
            },
            Self::error_callback,
            None
        ).unwrap();

        stream.play().unwrap();
        thread::sleep(Duration::from_secs(1));
        stream.pause().unwrap();
    }

    #[inline]
    fn data_callback(
        data:  &mut [i16],
        state: &State,
        voice: &mut Voice
    ) {
        for frame in data.chunks_mut(state.channels) {
            let sample = i16::from_sample(voice.render(state.sample_rate));

            for sample_in_frame in frame.iter_mut() {
                *sample_in_frame = sample;
            }
        }
    }

    fn error_callback(error: StreamError) {
        eprintln!("\x1b[31;1merror:\x1b[0m {error}");
    }
}

struct State {
    channels:    usize,
    sample_rate: f32
}

impl From<&StreamConfig> for State {
    fn from(config: &StreamConfig) -> Self {
        let channels    = config.channels      as usize;
        let sample_rate = config.sample_rate.0 as f32;

        Self { channels, sample_rate }
    }
}

