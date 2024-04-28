use crate::song::SongEvent;
use crate::TransformUniform;
use midly::MidiMessage;
use wgpu_jumpstart::wgpu;
use wgpu_jumpstart::Gpu;

mod pipeline;
use pipeline::SheetPipeline;
use wgpu_jumpstart::Uniform;

pub struct SheetRenderer {
    sheet_pipeline: SheetPipeline,
}

impl SheetRenderer {
    pub fn new(gpu: &Gpu, transform_uniform: &Uniform<TransformUniform>) -> Self {
        let sheet_pipeline = SheetPipeline::new(gpu, transform_uniform);
        Self { sheet_pipeline }
    }

    pub fn update_time(&mut self, gpu: &mut Gpu) {
        self.sheet_pipeline.update_time(gpu);
    }

    pub fn user_midi_event(&mut self, message: &MidiMessage) {
        let (is_on, midi_key) = match message {
            MidiMessage::NoteOn { key, .. } => (true, key.as_int()),
            MidiMessage::NoteOff { key, .. } => (false, key.as_int()),
            _ => return,
        };

        if is_on {
            let note = self.midi2note(midi_key);
            let holes = self.note2holes(note);
            for i in (0..6).rev() {
                let h: u16 = 1 << i;
                let hole = format!("fingerhole-{}", (6 - i));
                self.sheet_pipeline
                    .fingerhole_states_mut()
                    .entry(hole.into())
                    .and_modify(|fingerhole| match holes & h == h {
                        true => fingerhole.set_active(),
                        false => fingerhole.set_inactive(),
                    });
            }
        }
    }

    pub fn render<'rpass>(
        &'rpass mut self,
        transform_uniform: &'rpass Uniform<TransformUniform>,
        render_pass: &mut wgpu::RenderPass<'rpass>,
    ) {
        self.sheet_pipeline.render(transform_uniform, render_pass);
    }

    pub fn note2holes(&self, note: &str) -> u16 {
        match note {
            "D" => 0b0_111111,
            "E" => 0b0_111110,
            "F#" => 0b0_111100,
            "G" => 0b0_111000,
            "A" => 0b0_110000,
            "B" => 0b0_100000,
            "C" => 0b0_000000,
            "D'" => 0b1_011111,
            "E'" => 0b1_111111,
            "F#'" => 0b1_111110,
            "G'" => 0b1_111000,
            "A'" => 0b1_110000,
            "B'" => 0b1_100000,
            _ => 0,
        }
    }

    pub fn midi2note(&self, midi_key: u8) -> &str {
        match midi_key {
            62 => "D",
            64 => "E",
            66 => "F#",
            67 => "G",
            69 => "A",
            71 => "B",
            72 => "C",
            74 => "D'",
            76 => "E'",
            78 => "F#'",
            79 => "G'",
            81 => "A'",
            83 => "B'",
            _ => "?",
        }
    }

    pub fn song_events(&mut self, events: &[&SongEvent]) {
        for e in events {
            let (is_on, _) = match e.message {
                MidiMessage::NoteOn { key, .. } => (true, key.as_int()),
                MidiMessage::NoteOff { key, .. } => (false, key.as_int()),
                _ => continue,
            };
            self.sheet_pipeline
                .notehead_states_mut()
                .entry(e.notehead_id.to_string())
                .and_modify(|note| match is_on {
                    true => note.set_active(),
                    false => note.set_inactive(),
                });
        }
    }
}
