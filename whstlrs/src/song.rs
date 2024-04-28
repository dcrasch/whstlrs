use std::path::Path;
use std::sync::Arc;
use std::{fs, time::Duration};

use midly::{num::u7, MidiMessage};

#[derive(Debug, Clone)]
pub struct SongNote {
    pub timestamp: f32,
    pub midi_key: u8,
    pub duration: u32,
    pub duration_length: f32,
    pub notehead_id: String,
}

#[derive(Debug, Clone)]
pub struct SongEvent {
    pub channel: u8,
    pub timestamp: Duration,
    pub midi_key: u8,
    pub duration_length: f32,
    pub notehead_id: String,
    pub message: MidiMessage,
}

pub struct PlaybackState {
    song: Arc<Song>,
    song_state: Box<SongState>,
    running: Duration,
}
struct SongState {
    seen_events: usize,
}

impl PlaybackState {
    pub fn new(song: Arc<Song>) -> Self {
        PlaybackState {
            song,
            song_state: SongState { seen_events: 0 }.into(),
            running: Duration::ZERO,
        }
    }
    pub fn update(&mut self, delta: Duration) -> Vec<&SongEvent> {
        self.running += delta;
        let events = self.song.file.events[self.song_state.seen_events..]
            .iter()
            .take_while(|event| event.timestamp <= self.running)
            .inspect(|_| self.song_state.seen_events += 1)
            .collect();

        events
    }
}

#[derive(Debug, Clone)]
pub struct Song {
    pub file: SongFile,
    // SVG
}

impl Song {
    pub fn new(file: SongFile) -> Self {
        Self { file }
    }
}

#[derive(Debug, Clone)]
pub struct SongFile {
    pub name: String,
    pub notes: Vec<SongNote>,
    pub events: Vec<SongEvent>,
}

impl SongFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let name = path
            .as_ref()
            .file_name()
            .ok_or(String::from("File not found"))?
            .to_string_lossy()
            .to_string();

        let text: String = match fs::read_to_string(path) {
            Ok(buff) => buff,
            Err(_) => return Err(String::from("Could Not Open File")),
        };

        Self::from_str(&text, name)
    }

    pub fn from_str(text: &str, name: String) -> Result<Self, String> {
        // maybe parse using nom?
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(text.as_bytes());
        let mut notes: Vec<SongNote> = Vec::new();
        let mut events: Vec<SongEvent> = Vec::new();
        for record in reader.records() {
            if let Ok(record) = record {
                match &record[1] {
                    "note" => {
                        let timestamp = record[0].parse::<f32>().expect("duration length");
                        let duration_length = record[4].parse::<f32>().expect("duration length");
                        let midi_key = record[2].parse::<u8>().expect("pitch");
                        let duration = record[3].parse::<u32>().expect("duration");
                        let notehead_id = record[5].to_string();

                        let note = SongNote {
                            timestamp,
                            midi_key,
                            duration,
                            duration_length,
                            notehead_id: notehead_id.to_string(),
                        };
                        notes.push(note);

                        let timestamp_on = std::time::Duration::from_secs_f32(timestamp);
                        let timestamp_off =
                            std::time::Duration::from_secs_f32(timestamp + duration_length);
                        let event = SongEvent {
                            channel: 0,
                            timestamp: timestamp_on,
                            message: MidiMessage::NoteOn {
                                key: u7::new(midi_key),
                                vel: u7::new(127),
                            },
                            midi_key,
                            duration_length,
                            notehead_id: notehead_id.to_string(),
                        };
                        events.push(event);
                        let event = SongEvent {
                            channel: 0,
                            timestamp: timestamp_off,
                            message: MidiMessage::NoteOff {
                                key: u7::new(midi_key),
                                vel: u7::new(0),
                            },
                            midi_key,
                            duration_length,
                            notehead_id: notehead_id.to_string(),
                        };
                        events.push(event);
                    }
                    _ => (),
                }
            }
        }
        Ok(Self {
            name: name.to_string(),
            notes,
            events,
        })
    }
}
