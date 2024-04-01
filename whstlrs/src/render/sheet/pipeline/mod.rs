mod instance_data;

use wgpu_jumpstart::{
    wgpu, Gpu, RenderPipelineBuilder, Shape, TransformUniform, Uniform,
};

use bytemuck::{Pod, Zeroable};

pub struct SheetPipeline {
    render_pipeline: wgpu::RenderPipeline,

    quad: Shape,

    time_uniform: Uniform<TimeUniform>,
}

impl<'a> SheetPipeline {
    pub fn new(
        gpu: &Gpu,
        transform_uniform: &Uniform<TransformUniform>,
    ) -> Self {
        let shader = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("RectanglePipeline::shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "./shader.wgsl"
                ))),
            });

        let time_uniform = Uniform::new(
            &gpu.device,
            TimeUniform::default(),
            wgpu::ShaderStages::VERTEX,
        );

        let render_pipeline_layout =
            &gpu.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[
                        &transform_uniform.bind_group_layout,
                        &time_uniform.bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let target = wgpu_jumpstart::default_color_target_state(gpu.texture_format);

        let render_pipeline = wgpu::RenderPipelineDescriptor::builder(
            render_pipeline_layout,
            wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Shape::layout()],
            },
        )
        .fragment("fs_main", &shader, &[Some(target)])
        .create_render_pipeline(&gpu.device);

        let quad = Shape::new_quad(&gpu.device);
        Self {
            render_pipeline,
            quad,
            time_uniform,
        }
    }

    pub fn render(
        &'a self,
        transform_uniform: &'a Uniform<TransformUniform>,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &transform_uniform.bind_group, &[]);
        render_pass.set_bind_group(1, &self.time_uniform.bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.quad.vertex_buffer.slice(..));

        render_pass.set_index_buffer(self.quad.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct TimeUniform {
    time: f32,
    speed: f32,
}

impl Default for TimeUniform {
    fn default() -> Self {
        Self {
            time: 0.0,
            speed: 400.0,
        }
    }
}