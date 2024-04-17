use std::time::Duration;

use midly::MidiMessage;
use wgpu_jumpstart::{wgpu, TransformUniform, Uniform};

use crate::{
    context::Context,
    song::{Song, SongEvent},
};

use super::Scene;
use crate::render::SheetRenderer;

pub struct PlayingScene {
    sheet: SheetRenderer,
    song: Song,
}

impl PlayingScene {
    pub fn new(ctx: &Context, song: Song) -> Self {
        let mut sheet = SheetRenderer::new(&ctx.gpu, &ctx.transform);
        Self {
            sheet,
            song: song.clone(),
        }
    }

    fn update_song_player(&mut self, ctx: &Context, delta: Duration) -> f32 {
        let d = delta.as_secs_f32() / 8.0;
        let events: Vec<SongEvent> = self
            .song
            .file
            .notes
            .iter()
            .map(|n| {
                if d >= n.timestamp && d <= (n.timestamp + n.duration_length) {
                    SongEvent {
                        timestamp: n.timestamp,
                        midi_key: n.midi_key,
                        duration_length: n.duration_length,
                        notehead_id: n.notehead_id.to_string(),
                        wrong: false,
                    }
                } else {
                    SongEvent {
                        timestamp: n.timestamp,
                        midi_key: n.midi_key,
                        duration_length: n.duration_length,
                        notehead_id: n.notehead_id.to_string(),
                        wrong: true,
                    }
                }
            })
            .collect();
        self.sheet.song_events(&events);
        d
    }
}

impl Scene for PlayingScene {
    fn update(&mut self, ctx: &mut Context, delta: std::time::Duration) {
        self.sheet.update_time(&mut ctx.gpu, delta);

        let time = self.update_song_player(ctx, delta);
    }

    fn render<'rpass>(
        &'rpass mut self,
        transform_uniform: &'rpass Uniform<TransformUniform>,
        render_pass: &mut wgpu::RenderPass<'rpass>,
    ) {
        self.sheet.render(transform_uniform, render_pass);
    }

    fn window_event(&mut self, _ctx: &mut Context, _event: &winit::event::WindowEvent) {}
}
