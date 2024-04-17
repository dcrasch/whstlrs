use std::collections::HashMap;
use std::time::Duration;

use crate::song::SongEvent;
use crate::TransformUniform;
use wgpu_jumpstart::wgpu;
use wgpu_jumpstart::Gpu;

mod pipeline;
use pipeline::SheetPipeline;
use wgpu_jumpstart::Uniform;

use self::pipeline::NoteHeadState;

pub struct SheetRenderer {
    sheet_pipeline: SheetPipeline,
}

impl SheetRenderer {
    pub fn new(gpu: &Gpu, transform_uniform: &Uniform<TransformUniform>) -> Self {
        let sheet_pipeline = SheetPipeline::new(gpu, transform_uniform);
        Self { sheet_pipeline }
    }

    pub fn update_time(&mut self, gpu: &mut Gpu, delta: Duration) {
        self.sheet_pipeline.update_time(gpu, delta);
    }

    pub fn render<'rpass>(
        &'rpass mut self,
        transform_uniform: &'rpass Uniform<TransformUniform>,
        render_pass: &mut wgpu::RenderPass<'rpass>,
    ) {
        self.sheet_pipeline.render(transform_uniform, render_pass);
    }

    pub fn song_events(&mut self, events: &[SongEvent]) {
        for e in events {
            self.sheet_pipeline
                .notehead_states_mut()
                .entry(e.notehead_id.to_string())
                .and_modify(|note| {
                    if e.wrong {
                        note.set_inactive()
                    } else {
                        note.set_active()
                    }
                });
        }
    }
}
