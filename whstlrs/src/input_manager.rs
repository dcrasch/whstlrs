use midir::{MidiInput, MidiInputConnection};
use midly::{live::LiveEvent, MidiMessage};
use winit::event_loop::EventLoopProxy;

use crate::WhstlrsEvent;

pub struct InputConnection {
    tx: EventLoopProxy<WhstlrsEvent>,
    conn_in: Option<MidiInputConnection<()>>,
}

impl InputConnection {
    pub fn new(tx: EventLoopProxy<WhstlrsEvent>) -> Self {
        InputConnection { tx, conn_in: None }
    }

    pub fn connect_input(&mut self) {
        let tx = self.tx.clone();
        let midi_in = MidiInput::new("My Test Input").unwrap();
        let in_ports = midi_in.ports();
        let in_port = &in_ports[1];
        let conn_in = midi_in
            .connect(
                &in_port,
                "MidiIo-in-conn",
                move |_, message, _| {
                    let event = LiveEvent::parse(message).unwrap();

                    if let LiveEvent::Midi { channel, message } = event {
                        match message {
                            // Some keyboards send NoteOn event with vel 0 instead of NoteOff
                            midly::MidiMessage::NoteOn { key, vel } if vel == 0 => {
                                tx.send_event(WhstlrsEvent::MidiInput {
                                    channel: channel.as_int(),
                                    message: MidiMessage::NoteOff { key, vel },
                                })
                                .ok();
                            }
                            message => {
                                tx.send_event(WhstlrsEvent::MidiInput {
                                    channel: channel.as_int(),
                                    message,
                                })
                                .ok();
                            }
                        }
                    }
                },
                (),
            )
            .unwrap();
        self.conn_in = Some(conn_in);
    }
}
