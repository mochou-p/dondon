// mochou-p/dondon/src/audio.rs

use std::f32::consts;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use cpal::{OutputCallbackInfo, SampleFormat, SupportedStreamConfig};
use cpal::platform::{Stream, default_host};
use cpal::traits::{DeviceTrait, HostTrait};
use rtrb::Consumer;
use crate::Command;


pub static PLAYING: AtomicBool = AtomicBool::new(false);
pub static CLOCK:   AtomicU64  = AtomicU64 ::new(    0);

pub struct State {
    consumer:    Consumer<Command>,
    voices:      [Voice; 2],
    sample_rate: f32,
    playing:     bool
}

impl State {
    fn new(config: &SupportedStreamConfig, consumer: Consumer<Command>) -> Self {
        assert_eq!(         config.channels(),      2                 );
        assert_eq!(         config.sample_format(), SampleFormat::F32 );
        assert   !(matches!(config.sample_rate(),   44_100 | 48_000  ));

        let voices      = [Voice::new(440.0, 0.0, 2.0, Envelope::flat()), Voice::new(440.0, 4.0, 6.0, Envelope::default())];
        let sample_rate = config.sample_rate() as f32;
        let playing     = false;

        Self { consumer, voices, sample_rate, playing }
    }

    fn callback(mut self) -> impl FnMut(&mut [f32], &OutputCallbackInfo) {
        move |data, _info| {
            while let Ok(command) = self.consumer.pop() {
                match command {
                    Command::Resume => {
                        self.playing = true;
                        PLAYING.store(self.playing, Ordering::Relaxed);
                    },
                    Command::Pause => {
                        self.playing = false;
                        PLAYING.store(self.playing, Ordering::Relaxed);
                    }
                }
            }

            if !self.playing {
                return;
            }

            for frame in unsafe { data.as_chunks_unchecked_mut::<2>() } {
                frame[0] = 0.0;
                frame[1] = 0.0;

                let clock = CLOCK.fetch_add(1, Ordering::Relaxed);
                let time  = clock as f32 / self.sample_rate;

                for voice in self.voices.iter_mut() {
                    voice. render(time, frame);
                    voice.advance(self.sample_rate);
                }
            }
        }
    }
}

pub fn stream(consumer: Consumer<Command>) -> (Stream, f32) {
    let host        = default_host();
    let device      = host.default_output_device().unwrap();
    let config      = device.default_output_config().unwrap();

    let state       = State::new(&config, consumer);
    let sample_rate = state.sample_rate;

    let stream      = device.build_output_stream::<f32, _, _>(
        &config.config(),
        state.callback(),
        |err| eprintln!("\x1b[31mstream error:\x1b[0m {err}"),
        None
    ).unwrap();

    (stream, sample_rate)
}

#[expect(dead_code)]
enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth
}

impl Waveform {
    fn sample(&self, phase: f32) -> f32 {
        match self {
            Self::Sine     => (phase * consts::TAU).sin(),
            Self::Square   => -phase.round() * 2.0 + 1.0,
            Self::Triangle => {
                if      phase < 0.25 {  phase * 4.0       }
                else if phase < 0.75 { -phase * 4.0 + 2.0 }
                else                 {  phase       - 4.0 }
            },
            Self::Sawtooth => {
                if phase < 0.5 { phase * 2.0       }
                else           { phase * 2.0 - 2.0 }
            }
        }
    }
}

enum ADSR {
    Waiting,
    Attack(f32),
    Decay(f32),
    Sustain,
    Release(f32),
    Finished
}

struct Envelope {
    state:    ADSR,
    attack:   f32,
    decay:    f32,
    sustain:  f32,
    release:  f32
}

fn from_to_over(from: f32, to: f32, over: f32, time: f32) -> f32 {
    let normalized = time / over;
    from + (to - from) * normalized
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            state:   ADSR::Waiting,
            attack:  0.1,
            decay:   0.2,
            sustain: 1.0,
            release: 0.1
        }
    }
}

impl Envelope {
    fn flat() -> Self {
        Self {
            state:   ADSR::Waiting,
            attack:  0.0,
            decay:   0.0,
            sustain: 1.0,
            release: 0.0
        }
    }

    fn sample(&mut self, time: f32, end: f32) -> f32 {
        match self.state {
            ADSR::Waiting => {
                self.state = ADSR::Attack(time);
                self.sample(time, end)
            },
            ADSR::Attack(start) => {
                let elapsed = time - start;

                if elapsed >= self.attack {
                    self.state = ADSR::Decay(time);
                    self.sample(time, end)
                } else {
                    from_to_over(0.0, 1.0, self.attack, elapsed)
                }
            },
            ADSR::Decay(start) => {
                let elapsed = time - start;

                if elapsed >= self.decay {
                    self.state = ADSR::Sustain;
                    self.sample(time, end)
                } else {
                    from_to_over(1.0, self.sustain, self.decay, elapsed)
                }
            },
            ADSR::Sustain => {
                if time >= end {
                    self.state = ADSR::Release(time);
                    self.sample(time, end)
                } else {
                    self.sustain
                }
            },
            ADSR::Release(start) => {
                let elapsed = time - start;

                if elapsed >= self.release {
                    self.state = ADSR::Finished;
                    self.sample(time, end)
                } else {
                    from_to_over(self.sustain, 0.0, self.release, elapsed)
                }
            },
            ADSR::Finished => {
                0.0
            }
        }
    }
}

struct Voice {
    phase:     f32,
    frequency: f32,
    amplitude: f32,
    pan:       f32,
    shape:     Waveform,
    envelope:  Envelope,
    start:     f32,
    end:       f32
}

impl Voice {
    fn new(frequency: f32, start: f32, end: f32, envelope: Envelope) -> Self {
        Self {
            phase:     0.0,
            frequency,
            amplitude: 0.4,
            pan:       0.0,
            shape:     Waveform::Sine,
            envelope,
            start,
            end
        }
    }

    fn render(&mut self, time: f32, frame: &mut [f32]) {
        if
            (matches!(self.envelope.state, ADSR::Waiting) && time < self.start)
            ||
            matches!(self.envelope.state, ADSR::Finished)
        {
            return;
        }

        let         sample  = self.shape.sample(self.phase);
        let           gain  = self.envelope.sample(time, self.end) * self.amplitude;
        let normalized_pan  = (self.pan + 1.0) * 0.5;
        let          angle  = normalized_pan * consts::FRAC_PI_2;
        let      left_gain  = angle.cos().clamp(0.0, 1.0);
        let     right_gain  = angle.sin().clamp(0.0, 1.0);

        frame[0]           += sample *  left_gain * gain;
        frame[1]           += sample * right_gain * gain;
    }

    fn advance(&mut self, sample_rate: f32) {
        self.phase = (self.phase + (self.frequency / sample_rate)).fract();
    }
}

