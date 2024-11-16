extern crate midir;

use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;

use std::time::Instant;

// use super::session::{Channel, Note, Velocity};
use midir::{Ignore, MidiIO, MidiInput, MidiInputConnection};

pub type Channel = u8;
pub type Note = u8;
pub type Velocity = u8;

#[derive(Debug)]
pub struct MidiEvent {
    pub message: MidiMessage,
    pub instant: Option<Instant>,
}

#[derive(Debug)]
pub enum MidiMessageType {
    NoteOff,
    NoteOn,
    ControlChange,
}

#[derive(Debug)]
pub struct MidiMessage {
    pub r#type: MidiMessageType,
    pub channel: Channel,
    pub note: Note,
    pub velocity: Velocity,
}

impl MidiMessage {
    pub fn from_array(message: &[u8]) -> MidiMessage {
        println!("received {:?}", message);
        return MidiMessage {
            r#type: match message[0] {
                0x80..=0x8F => MidiMessageType::NoteOff,
                0x90..=0x9F => MidiMessageType::NoteOn,
                0xb0..=0xbf => MidiMessageType::ControlChange,
                _ => panic!("Unknown MIDI message {:?}", message), // TODO
            },
            channel: 1, // TODO
            note: message[1],
            velocity: message[2],
        };
    }
}

type MidiEventQueue = VecDeque<MidiEvent>;

pub struct Instrument {
    name: String,
    midi_in: Option<MidiInputConnection<mpsc::Sender<MidiEvent>>>,
    events_in: MidiEventQueue,
    chan_out: mpsc::Sender<MidiEvent>,
    chan_in: mpsc::Receiver<MidiEvent>,
    debug: bool,
}

impl Instrument {
    pub fn new(name: &str) -> Instrument {
        let (tx, rx) = mpsc::channel();
        Instrument {
            midi_in: None,
            events_in: VecDeque::new(),
            chan_out: tx,
            chan_in: rx,
            name: name.to_string(),
            debug: true,
        }
    }

    pub fn set_debug(&mut self, v: bool) {
        self.debug = v;
    }

    fn receive_events(&mut self) {
        loop {
            match self.chan_in.try_recv() {
                Ok(t) => self.events_in.push_back(t),
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => break,
                    mpsc::TryRecvError::Disconnected => panic!("Channel died"),
                },
            }
        }
    }

    pub fn has_events(&mut self) -> bool {
        self.receive_events();
        return self.events_in.len() > 0;
    }

    pub fn pop_event(&mut self) -> Option<MidiEvent> {
        self.receive_events();
        let element = self.events_in.pop_front();
        return element;
    }

    pub fn connect_in(&mut self, port: u8) {
        let mut midi_in = MidiInput::new("instrument").unwrap();
        midi_in.ignore(Ignore::None);
        let in_port = self.select_port(port, &midi_in).unwrap();
        let port_name = midi_in.port_name(&in_port).unwrap();
        println!("Connection open, incoming from '{}' ...", port_name);

        // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
        self.midi_in = Some(
            midi_in
                .connect(
                    &in_port,
                    "midir-forward",
                    |stamp, message, chan_out| {
                        // conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
                        println!("{}: {:?} (len = {})", stamp, message, message.len());
                        // let value : usize = message[1] as usize;
                        chan_out
                            .send(MidiEvent {
                                message: MidiMessage::from_array(message),
                                instant: None,
                            })
                            .unwrap();
                    },
                    self.chan_out.clone(),
                )
                .unwrap(),
        );
    }

    fn select_port<T: MidiIO>(&self, force: u8, midi_io: &T) -> Result<T::Port, ()> {
        // println!("Available {} ports:", descr);
        let midi_ports = midi_io.ports();
        // for (i, p) in midi_ports.iter().enumerate() {
        //     println!("{}: {}", i, midi_io.port_name(p)?);
        // }
        // print!("Please select {} port: ", descr);
        // stdout().flush()?;
        // let mut input = String::new();
        // stdin().read_line(&mut input)?;
        // let port = midi_ports.get(input.trim().parse::<usize>()?)
        //                      .ok_or("Invalid port number")?;
        let port = midi_ports
            .get(force as usize)
            .ok_or("Invalid port number")
            .unwrap();
        Ok(port.clone())
    }
}
