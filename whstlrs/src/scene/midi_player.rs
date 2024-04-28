use std::time::Duration;

use midly::num::u4;

use crate::{
    output_manager::OutputConnection,
    song::{PlaybackState, Song, SongEvent},
};

pub struct MidiPlayer {
    output: OutputConnection,
    playback: PlaybackState,
}
impl MidiPlayer {
    pub fn new(song: Song) -> Self {
        let output = OutputConnection::new();
        MidiPlayer {
            output,
            playback: PlaybackState::new(song.into()),
        }
    }

    pub fn update(&mut self, delta: Duration) -> Vec<&SongEvent> {
        let events = self.playback.update(delta);
        events.iter().for_each(|event| {
            self.output
                .midi_event(u4::new(event.channel), event.message);
        });
        events
    }
}
