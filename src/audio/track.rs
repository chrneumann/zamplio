use super::{sample::Sample, SampleFormat};
use anyhow;

use fundsp::prelude::*;

/// Audio track which plays at most one sample at a time.
pub struct Track {
    /// Sample to play.
    sample: Option<Sample>,
    /// Current position in sample.
    position: usize,
    /// Ping-pong loop the sample (reverse from end).
    pingpong: bool,
    /// Play reverse.
    reverse: bool,
    /// Currently playing?
    playing: bool,
    /// Start position.
    start: usize,
    /// Stop position.
    stop: usize,
    // filter: An<Unit<U2, U2>>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            sample: None,
            position: 0,
            reverse: false,
            pingpong: false,
            playing: false,
            start: 0,
            stop: 0,
            // filter: unit::<U1, U1>(Box::new(lowpass_hz(5000.0, 1.0) >> highpass_hz(1000.0, 1.0))),
            // filter: unit::<U2, U2>(Box::new(reverb_stereo(1.0, 0.2, 0.7))),
        }
    }

    /// Loads a sample into the track.
    pub fn load_sample(&mut self, path: &str) -> Result<(), anyhow::Error> {
        self.sample = Some(Sample::load(path)?);
        self.stop = self.sample.as_ref().unwrap().get_data().len() - 1;
        Ok(())
    }

    /// Strike the track / start a sample play.
    pub fn strike(&mut self) {
        self.position = self.start;
        self.playing = true;
    }

    /// Returns the next sample.
    pub fn tick(&mut self) -> Result<Option<f32>, anyhow::Error> {
        if !self.playing || self.position >= self.stop {
            return Ok(None);
        }
        let sample = self.sample.as_ref().unwrap().get_position(self.position);
        if self.pingpong {
            if !self.reverse {
                self.position += 1;
                if self.position > self.stop {
                    self.reverse = true;
                }
            } else {
                self.position -= 1;
                if self.position == self.start {
                    self.reverse = false;
                }
            }
        } else {
            if self.position > self.stop {
                self.position = self.start;
                self.playing = false;
            } else {
                self.position += 1;
            }
        }
        // let sample = self.filter.filter_stereo(sample, sample);
        Ok(Some(sample))
    }
}
