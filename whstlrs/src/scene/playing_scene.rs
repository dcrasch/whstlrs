use std::{f32::EPSILON, time::Duration};

use midly::{num::u7, MidiMessage};
use wgpu_jumpstart::{wgpu, TransformUniform, Uniform};

use crate::{
    context::Context,
    scene::midi_player::MidiPlayer,
    song::{Song, SongEvent},
};

use super::Scene;

use crate::render::SheetRenderer;

pub struct PlayingScene {
    sheet: SheetRenderer,
    player: MidiPlayer,
}

impl PlayingScene {
    pub fn new(ctx: &Context, song: Song) -> Self {
        let sheet = SheetRenderer::new(&ctx.gpu, &ctx.transform);

        let player = MidiPlayer::new(song);
        Self {
            sheet,
            player,
        }
    }

    fn update_song_player(&mut self, ctx: &Context, delta: Duration) -> f32 {

        let events = self.player.update(delta);
        self.sheet.song_events(&events);
        0.0
    }
}

impl Scene for PlayingScene {
    fn update(&mut self, ctx: &mut Context, delta: std::time::Duration) {
        self.sheet.update_time(&mut ctx.gpu);

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
