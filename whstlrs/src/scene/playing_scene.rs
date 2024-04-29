use std::time::Duration;

use crate::{context::Context, scene::midi_player::MidiPlayer, song::Song};
use midly::MidiMessage;
use wgpu_jumpstart::{wgpu, TransformUniform, Uniform};
use winit::{
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::{Key, NamedKey},
};

use super::Scene;

use crate::render::SheetRenderer;

pub struct PlayingScene {
    pub sheet: SheetRenderer,
    pub player: MidiPlayer,
}

impl PlayingScene {
    pub fn new(ctx: &Context, song: Song) -> Self {
        let sheet = SheetRenderer::new(&ctx.gpu, &ctx.transform);

        let player = MidiPlayer::new(song);
        Self { sheet, player }
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

    fn window_event(&mut self, ctx: &mut Context, event: &winit::event::WindowEvent) {
        SheetRenderer::handle_window_event(self, ctx, event);
    }

    fn midi_event(&mut self, _ctx: &mut Context, _channel: u8, message: &MidiMessage) {
        self.sheet.user_midi_event(&message);
    }
}
