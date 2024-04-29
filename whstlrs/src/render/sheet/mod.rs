use crate::scene::playing_scene::PlayingScene;
use crate::song::SongEvent;
use crate::song::SongNote;
use crate::Context;
use crate::TransformUniform;

use midly::MidiMessage;
use wgpu_jumpstart::wgpu;
use wgpu_jumpstart::Gpu;

mod pipeline;
use pipeline::SheetPipeline;
use wgpu_jumpstart::Uniform;
use winit::event::ElementState;
use winit::event::MouseButton;
use winit::event::WindowEvent;

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
            let note = SheetRenderer::midi2note(midi_key);
            let holes = SheetRenderer::note2holes(&note);
            //println!("{} {}", midi_key, note);
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
        } else {
            // mmmm
        }
    }

    pub fn render<'rpass>(
        &'rpass mut self,
        transform_uniform: &'rpass Uniform<TransformUniform>,
        render_pass: &mut wgpu::RenderPass<'rpass>,
    ) {
        self.sheet_pipeline.render(transform_uniform, render_pass);
    }

    pub fn note2holes(note: &str) -> u16 {
        match note {
            "D" => 0b0_111111,
            "E" => 0b0_111110,
            "F" => 0b0_111101, // warble halfnote
            "F#" => 0b0_111100,
            "G" => 0b0_111000,
            "G#" => 0b0_110111, // warble
            "A" => 0b0_110000,
            "A#" => 0b0_101111, // warble
            "B" => 0b0_100000,
            "C" => 0b0_011000,
            "C#" => 0b0_000000,

            "D'" => 0b1_011111,
            "E'" => 0b1_111110,
            "F'" => 0b1_111101, // warble
            "F#'" => 0b1_111100,
            "G'" => 0b1_111000,
            "G#" => 0b1_110111, // warble
            "A'" => 0b1_110000,
            "A#'" => 0b1_101111, // warble
            "B'" => 0b1_100000,
            "C'" => 0b1_011100,
            "C#'" => 0b1_000000,
            "D''" => 0b1_011111,
            _ => 0,
        }
    }

    pub fn midi2note(midi_key: u8) -> String {
        match midi_key {
            62 => "D",
            64 => "E",
            65 => "F",
            66 => "F#",
            67 => "G",
            68 => "G#",
            69 => "A",
            70 => "A#",
            71 => "B",
            72 => "C",
            73 => "C#",
            74 => "D'",

            76 => "E'",
            77 => "F'",
            78 => "F#'",
            79 => "G'",
            80 => "G#'",
            81 => "A'",
            82 => "A#'",
            83 => "B'",
            84 => "C'",
            85 => "C#'",
            86 => "D''",
            _ => "?",
        }
        .into()
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

    pub fn handle_window_event(
        scene: &mut PlayingScene,
        ctx: &mut Context,
        event: &WindowEvent,
    ) -> bool {
        match &event {
            WindowEvent::MouseInput { state, button, .. } => {

                return Self::handle_mouse_input(scene, ctx, state, button);
            }
            _ => {}
        }
        true
    }
    fn handle_mouse_input(
        scene: &mut PlayingScene,
        ctx: &mut Context,
        state: &ElementState,
        button: &MouseButton,
    ) -> bool {

        if  (state, button) == (&ElementState::Pressed, &MouseButton::Left) {
        let pos = &ctx.window_state.cursor_logical_position;
        let h = ctx.window_state.logical_size.width;
        let w = ctx.window_state.logical_size.width;
        let x= pos.x;
        let y = pos.y;
        if let Some(notehead_id) = scene.sheet.sheet_pipeline.notehead_match(x, y) {
            let note = ctx
            .song
            .as_ref()
            .unwrap()
            .file
            .notes
            .iter()
            .find(|note| note.notehead_id == notehead_id);
            if let Some(&SongNote { midi_key, ..}) =  note {
                let note = SheetRenderer::midi2note(midi_key);
                let holes = SheetRenderer::note2holes(&note);
                //println!("{} {}", midi_key, note);
                for i in (0..6).rev() {
                    let h: u16 = 1 << i;
                    let hole = format!("fingerhole-{}", (6 - i));
                    scene
                        .sheet
                        .sheet_pipeline
                        .fingerhole_states_mut()
                        .entry(hole.into())
                        .and_modify(|fingerhole| match holes & h == h {
                            true => fingerhole.set_active(),
                            false => fingerhole.set_inactive(),
                        });
                }
                return true;
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
        
    }
    return false;
}
    }
