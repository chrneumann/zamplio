use super::{track::Track, Audio};
use anyhow;

// Main sampler struct.
pub struct Sampler {
    audio: Audio,
    tracks: Vec<Track>,
}

impl Sampler {
    pub fn new() -> Result<Self, anyhow::Error> {
        let mut tracks = vec![Track::new()];

        tracks[0].load_sample("example.wav")?;

        let mut audio = Audio::new();
        audio.init()?;

        Ok(Sampler { audio, tracks })
    }

    /// Returns the given track.
    pub fn get_track_mut(&mut self, index: usize) -> &mut Track {
        &mut self.tracks[index]
    }

    /// Next step in the loop.
    pub fn tick(&mut self) -> Result<(), anyhow::Error> {
        while !self.audio.buffer_full() {
            let mut anything_new = false;
            match self.tracks[0].tick()? {
                Some(sample) => {
                    self.audio.play_sample(sample)?;
                    anything_new = true;
                }
                None => {}
            }
            if !anything_new {
                return Ok(());
            }
        }
        Ok(())
    }
}
