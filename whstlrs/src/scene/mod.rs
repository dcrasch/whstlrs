pub mod midi_player;
pub mod playing_scene;

use crate::context::Context;
use std::time::Duration;
use midly::MidiMessage;
use wgpu_jumpstart::{wgpu, TransformUniform, Uniform};
use winit::event::WindowEvent;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context, delta: Duration);
    fn render<'pass>(
        &'pass mut self,
        transform: &'pass Uniform<TransformUniform>,
        rpass: &mut wgpu::RenderPass<'pass>,
    );
    fn window_event(&mut self, _ctx: &mut Context, _event: &WindowEvent) {}
    fn midi_event(&mut self, ctx: &mut Context, channel: u8, message: &MidiMessage) {
    }
}
