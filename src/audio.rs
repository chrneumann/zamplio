extern crate anyhow;
extern crate clap;
extern crate cpal;

pub mod sample;
pub mod sampler;
pub mod track;

pub type SampleFormat = f32;

use anyhow::anyhow;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SizedSample,
};
use cpal::{FromSample, SampleRate};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Interface to the systems audio.
pub struct Audio {
    stream: Option<cpal::Stream>,
    buffer: Arc<Mutex<VecDeque<SampleFormat>>>,
}

impl Audio {
    pub fn new() -> Self {
        Audio {
            stream: None,
            buffer: Arc::new(Mutex::new(VecDeque::<SampleFormat>::new())),
        }
    }

    pub fn init(&mut self) -> Result<(), anyhow::Error> {
        let (_host, device, config) = host_device_setup()?;
        self.stream = Some(match config.sample_format() {
            cpal::SampleFormat::F32 => self.make_stream::<f32>(&device, &config.into()),
            sample_format => Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            ))),
        }?);
        Ok(())
    }

    /// Returns true if the audio buffer is full.
    pub fn buffer_full(&self) -> bool {
        self.buffer.lock().unwrap().len() > 5000
    }

    pub fn make_stream<T>(
        &mut self,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
    ) -> Result<cpal::Stream, anyhow::Error>
    where
        T: SizedSample + FromSample<SampleFormat>,
    {
        let num_channels = config.channels as usize;
        let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

        let time_at_start = std::time::Instant::now();
        println!("Time at start: {:?}", time_at_start);

        let buffer = self.buffer.clone();

        let stream = device.build_output_stream(
            config,
            move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                // let mut rng = rand::thread_rng();
                for frame in output.chunks_mut(num_channels) {
                    let left_value: T = {
                        let mut buffer_guard = buffer.lock().unwrap();
                        buffer_guard
                            .pop_front()
                            .map(T::from_sample)
                            .unwrap_or_else(|| T::from_sample(0.0))
                    };
                    let right_value: T = {
                        let mut buffer_guard = buffer.lock().unwrap();
                        buffer_guard
                            .pop_front()
                            .map(T::from_sample)
                            .unwrap_or_else(|| T::from_sample(0.0))
                    };
                    let mut i = 0;
                    for sample in frame.iter_mut() {
                        if i == 0 {
                            *sample = left_value.into();
                        } else {
                            *sample = right_value.into();
                        }
                        i += 1;
                    }
                }
            },
            err_fn,
            None,
        )?;

        Ok(stream)
    }

    pub fn play_sample(&mut self, sample: SampleFormat) -> anyhow::Result<()> {
        self.buffer.lock().unwrap().push_back(sample);
        self.stream.as_ref().unwrap().play()?;
        Ok(())
    }
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    for range in device.supported_output_configs()? {
        println!("config {:?}", range);
        if range.channels() != 2 {
            continue;
        }
        match range.sample_format() {
            cpal::SampleFormat::F32 => {
                return Ok((host, device, range.with_sample_rate(SampleRate(48000))));
            }
            _ => {}
        }
    }
    Err(anyhow!("No suitable audio config found"))
}
