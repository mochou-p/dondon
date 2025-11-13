mod note;
mod oscillator;
mod voice;
mod waveform;

use std::thread;
use std::time::Duration;

use cpal::{Device, Sample, SampleFormat, StreamConfig, StreamError, SupportedStreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use note::Note;
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
        const ASSUMED_SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;

        let sample_format = self.supported.sample_format();

        if sample_format != ASSUMED_SAMPLE_FORMAT {
            eprintln!("\x1b[33;1mwarning:\x1b[0m your default sample format is {sample_format}, but this forces {ASSUMED_SAMPLE_FORMAT}");
        }

        let     config = self.supported.config();
        let     state  = State::from(&config);
        let mut voices = vec![
            Voice::from(Note::D(3)),
            Voice::from(Note::E(3)),
            Voice::from(Note::A(3)),
            Voice::from(Note::C(4))
        ];

        let stream = self.device.build_output_stream(
            &config,
            move |data, _| {
                Self::data_callback(data, &state, &mut voices);
            },
            Self::error_callback,
            None
        ).unwrap();

        stream.play().unwrap();
        thread::sleep(Duration::from_secs(2));
        stream.pause().unwrap();
    }

    #[inline]
    fn data_callback(
        data:   &mut [f32],
        state:  &State,
        voices: &mut [Voice]
    ) {
        for sample in data.iter_mut() {
            *sample = f32::EQUILIBRIUM;
        }

        for voice in voices.iter_mut() {
            voice.render_and_mix(data, state);
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
        const ASSUMED_CHANNEL_COUNT: usize = 2;

        let channels    = config.channels      as usize;
        let sample_rate = config.sample_rate.0 as f32;

        if channels != ASSUMED_CHANNEL_COUNT {
            eprintln!("\x1b[33;1mwarning:\x1b[0m your default channel count is {channels}, but this has only been tested on {ASSUMED_CHANNEL_COUNT} channels");
        }

        Self { channels, sample_rate }
    }
}

