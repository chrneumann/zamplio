mod audio;
mod midi;

use audio::sampler::Sampler;
use midi::Instrument;

use midi_msg::*;

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
                    match event.message {
                        MidiMsg::ChannelVoice { channel: _, msg } => match msg {
                            ChannelVoiceMsg::NoteOn { note: _, velocity } => {
                                if velocity > 0 {
                                    sampler.get_track_mut(0).strike();
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    };
                }
                None => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(100));
    }
    Ok(())
}
