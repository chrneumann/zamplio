use super::{sample::Sample, SampleFormat};
use anyhow;

/// Audio track which plays at most one sample at a time.
pub struct Track {
    /// Sample to play.
    sample: Option<Sample<SampleFormat>>,
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
            stop: 26000,
        }
    }

    /// Loads a sample into the track.
    pub fn load_sample(&mut self, path: &str) -> Result<(), anyhow::Error> {
        self.sample = Some(Sample::<SampleFormat>::load(path)?);
        Ok(())
    }

    /// Strike the track / start a sample play.
    pub fn strike(&mut self) {
        self.position = self.start;
        self.playing = true;
    }

    /// Returns the next sample.
    pub fn tick(&mut self) -> Result<Option<SampleFormat>, anyhow::Error> {
        if !self.playing {
            return Ok(None);
        }
        let sample = self.sample.as_ref().unwrap().get_data()[self.position];
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
        Ok(Some(sample))
    }
}
