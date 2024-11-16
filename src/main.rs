mod audio;
mod midi;

use audio::sampler::Sampler;
use midi::Instrument;

use crate::midi::MidiMessageType;

fn main() -> anyhow::Result<()> {
    let mut sampler = Sampler::new()?;
    let mut instrument = Instrument::new("pad");
    instrument.connect_in(2);
    loop {
        sampler.tick()?;
        if instrument.has_events() {
            match instrument.pop_event() {
                Some(event) => {
                    println! {"event {:?}", event};
                    match event.message.r#type {
                        MidiMessageType::NoteOn => {
                            sampler.get_track_mut(0).strike();
                        }
                        _ => {}
                    }
                }
                None => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(100));
    }
    Ok(())
}
