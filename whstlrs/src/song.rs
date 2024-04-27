use std::path::Path;
use std::{fs, time::Duration};

use midly::{num::u4, MidiMessage, TrackEvent, TrackEventKind};

#[derive(Debug, Clone)]
pub struct MidiEvent {
    pub channel: u8,
    pub timestamp: Duration,
    pub message: MidiMessage,
    pub track_id: usize,
    pub track_color_id: usize,
}

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
    pub timestamp: f32,
    pub midi_key: u8,
    pub duration_length: f32,
    pub notehead_id: String,
    pub wrong: bool,
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
        for record in reader.records() {
            if let Ok(record) = record {
                match &record[1] {
                    "note" => {
                        let timestamp = record[0].parse::<f32>().expect("duration length");
                        let duration_length = record[4].parse::<f32>().expect("duration length");
                        let note = SongNote {
                            timestamp,
                            midi_key: record[2].parse::<u8>().expect("pitch"),
                            duration: record[3].parse::<u32>().expect("duration"),
                            duration_length,
                            notehead_id: record[5].to_string(),
                        };
                        notes.push(note);
                    }
                    _ => (),
                }
            }
        }
        Ok(Self {
            name: name.to_string(),
            notes,
        })
    }
}
