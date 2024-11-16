mod audio;

use audio::sample::Sample;
use audio::{Audio, SampleFormat};

fn main() -> anyhow::Result<()> {
    let mut audio = Audio::new();
    audio.init()?;
    let sample = Sample::<SampleFormat>::load("example.wav")?;
    std::thread::sleep(std::time::Duration::from_millis(2000));
    println! {"go"};
    audio.play_sample(&sample)?;
    std::thread::sleep(std::time::Duration::from_millis(2000));
    audio.play_sample(&sample)?;
    std::thread::sleep(std::time::Duration::from_millis(10000));
    Ok(())
}
