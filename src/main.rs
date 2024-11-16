mod audio;

use audio::sampler::Sampler;

fn main() -> anyhow::Result<()> {
    let mut sampler = Sampler::new()?;
    loop {
        sampler.tick()?;
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    // std::thread::sleep(std::time::Duration::from_millis(10000));
    Ok(())
}
