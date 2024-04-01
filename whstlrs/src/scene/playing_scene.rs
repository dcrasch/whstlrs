use wgpu_jumpstart::{wgpu, TransformUniform, Uniform};

use crate::context::Context;

use super::Scene;
use crate::render::SheetRenderer;

pub struct PlayingScene {
    sheet : SheetRenderer
}

impl PlayingScene {
    
    pub fn new(ctx: &Context) -> Self {
        let mut sheet = SheetRenderer::new(
            &ctx.gpu,
            &ctx.transform,
        );
        Self {
            sheet,
        }

    }
}

impl Scene for PlayingScene {
    fn update(&mut self, ctx: &mut Context, delta: std::time::Duration) {
        todo!()
    }


    fn render<'rpass>(
        &'rpass mut self,
        transform_uniform: &'rpass Uniform<TransformUniform>,
        render_pass: &mut wgpu::RenderPass<'rpass>,
    ) {
        self.sheet.render(transform_uniform, render_pass);
    }
}