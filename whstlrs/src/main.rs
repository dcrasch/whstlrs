mod context;
mod render;
mod scene;
mod song;
mod utils;

use crate::context::Context;

use scene::{playing_scene, Scene};
use std::sync::Arc;
use std::time::Duration;
use utils::window::WindowState;
use wgpu_jumpstart::wgpu::TextureView;
use wgpu_jumpstart::{wgpu, Surface};
use wgpu_jumpstart::{Gpu, TransformUniform};
use winit::{
    event::WindowEvent,
    event_loop::{EventLoop, EventLoopBuilder},
};

#[derive(Debug)]
pub enum WhstlrsEvent {
    Exit,
}

struct Whstlrs {
    context: Context,
    surface: Surface,
    msaa_texture: Option<TextureView>,
    game_scene: Box<dyn Scene>,
    last_time: std::time::Instant,
}

impl Whstlrs {
    fn new(mut context: Context, surface: Surface) -> Self {
        let whistletab_scene =
            playing_scene::PlayingScene::new(&context, context.song.clone().unwrap());
        context.resize();
        context.gpu.submit();

        let mut msaa_texture = Some(
            context
                .gpu
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("Multisampled frame descriptor"),
                    size: wgpu::Extent3d {
                        width: 1080,
                        height: 720,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 4,
                    dimension: wgpu::TextureDimension::D2,
                    format: context.gpu.texture_format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );
        //msaa_texture = None;

        Self {
            context,
            surface,
            msaa_texture: msaa_texture,
            game_scene: Box::new(whistletab_scene),
            last_time: std::time::Instant::now(),
        }
    }
    fn whstlrs_event(
        &mut self,
        event: WhstlrsEvent,
        event_loop: &winit::event_loop::EventLoopWindowTarget<WhstlrsEvent>,
    ) {
    }

    fn window_event(
        &mut self,
        event: &WindowEvent,
        event_loop: &winit::event_loop::EventLoopWindowTarget<WhstlrsEvent>,
    ) {
        self.context.window_state.window_event(event);

        match &event {
            WindowEvent::Resized(_) => {
                let mut msaa_texture = Some(
                    self.context
                        .gpu
                        .device
                        .create_texture(&wgpu::TextureDescriptor {
                            label: Some("Multisampled frame descriptor"),
                            size: wgpu::Extent3d {
                                width: self.context.window_state.physical_size.width,
                                height: self.context.window_state.physical_size.height,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 4,
                            dimension: wgpu::TextureDimension::D2,
                            format: self.context.gpu.texture_format,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            view_formats: &[],
                        })
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                );
                self.msaa_texture = msaa_texture;

                self.surface.resize_swap_chain(
                    &self.context.gpu.device,
                    self.context.window_state.physical_size.width,
                    self.context.window_state.physical_size.height,
                );

                self.context.resize();

                self.context.gpu.submit();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                // TODO: Check if this update is needed;
                self.context.resize();
            }
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        state: winit::event::ElementState::Pressed,
                        logical_key,
                        ..
                    },
                ..
            } => match logical_key {
                winit::keyboard::Key::Character(c) if c.as_str() == "f" => {
                    if self.context.window.fullscreen().is_some() {
                        self.context.window.set_fullscreen(None);
                    } else {
                        let monitor = self.context.window.current_monitor();
                        if let Some(monitor) = monitor {
                            let f = winit::window::Fullscreen::Borderless(Some(monitor));
                            self.context.window.set_fullscreen(Some(f));
                        } else {
                            let f = winit::window::Fullscreen::Borderless(None);
                            self.context.window.set_fullscreen(Some(f));
                        }
                    }
                }
                _ => {}
            },
            WindowEvent::RedrawRequested => {
                let delta = self.last_time.elapsed();

                self.update(delta);
                self.render();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn update(&mut self, delta: Duration) {
        self.game_scene.update(&mut self.context, delta);
    }

    fn render(&mut self) {
        let frame = loop {
            let swap_chain_output = self.surface.get_current_texture();
            match swap_chain_output {
                Ok(s) => break s,
                Err(err) => log::warn!("deze {:?}", err),
            }
        };

        let view = &frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let bg_color = wgpu_jumpstart::Color::new(1.0, 1.0, 1.0, 0.0).into_linear_wgpu_color();
            let mut rpass =
                self.context
                    .gpu
                    .encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Main Whstlrs Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: self.msaa_texture.as_ref().unwrap_or(view),
                            resolve_target: if self.msaa_texture.is_some() {
                                Some(&view)
                            } else {
                                None
                            },
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(bg_color),
                                store: wgpu::StoreOp::Store,
                            },
                        })],

                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
            self.game_scene.render(&self.context.transform, &mut rpass);
        }

        self.context.gpu.submit();
        frame.present();
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("whstlrs=info"))
        .init();

    let event_loop: EventLoop<WhstlrsEvent> = EventLoopBuilder::with_user_event().build().unwrap();

    let builder = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize {
            width: 1080.0,
            height: 720.0,
        })
        .with_title("Whistlrs")
        .with_theme(Some(winit::window::Theme::Dark));

    let window = builder.build(&event_loop).unwrap();

    let window_state = WindowState::new(&window);
    let size = window.inner_size();
    let window = Arc::new(window);
    let (gpu, surface) =
        futures::executor::block_on(Gpu::for_window(window.clone(), size.width, size.height))
            .unwrap();

    let ctx = Context::new(window, window_state, event_loop.create_proxy(), gpu);

    let mut app = Whstlrs::new(ctx, surface);

    event_loop
        .run(move |event, event_loop| {
            use winit::event::Event;
            match event {
                Event::UserEvent(event) => {
                    app.whstlrs_event(event, event_loop);
                }
                Event::WindowEvent { event, .. } => {
                    app.window_event(&event, event_loop);
                }
                Event::AboutToWait => {
                    app.context.window.request_redraw();
                }
                _ => {}
            }
        })
        .unwrap();
}
