use crate::TransformUniform;
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


    pub fn update(&mut self, queue: &wgpu::Queue, time: f32) {
        self.sheet_pipeline.update_time(queue, time);
    }

    pub fn render<'rpass>(
        &'rpass mut self,
        transform_uniform: &'rpass Uniform<TransformUniform>,
        render_pass: &mut wgpu::RenderPass<'rpass>,
    ) {
        self.sheet_pipeline.render(transform_uniform, render_pass);
    }
}
