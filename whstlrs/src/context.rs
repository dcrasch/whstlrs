use crate::utils::window::WindowState;
use crate::WhstlrsEvent;
use std::sync::Arc;
use wgpu_jumpstart::{wgpu, Gpu, TransformUniform, Uniform};
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
pub struct Context {
    pub window: Arc<Window>,
    pub window_state: WindowState,
    pub gpu: Gpu,

    pub transform: Uniform<TransformUniform>,
    pub proxy: EventLoopProxy<WhstlrsEvent>,
}

impl Context {
    pub fn new(
        window: Arc<Window>,
        window_state: WindowState,
        proxy: EventLoopProxy<WhstlrsEvent>,
        gpu: Gpu,
    ) -> Self {
        let transform_uniform = Uniform::new(
            &gpu.device,
            TransformUniform::default(),
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        );

        Self {
            window,
            window_state,
            gpu,
            transform: transform_uniform,
            proxy,
        }
    }

    pub fn resize(&mut self) {
        self.transform.data.update(
            self.window_state.logical_size.width,
            self.window_state.logical_size.height,
            self.window_state.scale_factor as f32,
        );
        self.transform.update(&self.gpu.queue);
    }
}
