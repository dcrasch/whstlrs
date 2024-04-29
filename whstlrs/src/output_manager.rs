use midir::{MidiOutput, MidiOutputConnection};
use midly::{num::u4, MidiMessage};

pub struct OutputConnection {
    conn_out: MidiOutputConnection,
}

impl OutputConnection {
    pub fn new() -> Self {
        let midi_out = MidiOutput::new("Whstlrs").unwrap();
        let out_ports = midi_out.ports();
        for (i, p) in out_ports.iter().enumerate() {
            log::info!("{}: {}", i, midi_out.port_name(p).unwrap());
        }
        let out_port = &out_ports[0];
        let conn_out = midi_out.connect(out_port, "whstlrs-out").unwrap();
        OutputConnection { conn_out }
    }
    pub fn midi_event(&mut self, _channel: u4, msg: MidiMessage) {
        let (msg, key, velocity) = match msg {
            MidiMessage::NoteOn { vel, key } => (0x90, key.as_int(), vel.as_int()),
            MidiMessage::NoteOff { vel, key } => (0x80, key.as_int(), vel.as_int()),
            _ => {
                return;
            }
        };
        self.conn_out.send(&[msg, key, velocity]).unwrap();
    }
}
