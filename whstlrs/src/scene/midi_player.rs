use midly::{num::u4, MidiMessage};
use std::time::Duration;

use crate::{
    output_manager::OutputConnection,
    song::{MidiEvent, Song, SongEvent},
};

pub struct MidiPlayer {
    output: OutputConnection,
}
impl MidiPlayer {
    pub fn new() -> Self {
        let output = OutputConnection::new();
        MidiPlayer { output }
    }

    pub fn song_events(&mut self, messages: &[MidiMessage]) {
        for message in messages {
            println!("{:#?}", message);
            self.output.midi_event(u4::new(1), *message);
        }
    }
}
