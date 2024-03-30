mod context;
mod utils;

use crate::context::Context;

use std::sync::Arc;
use utils::window::WindowState;
use wgpu_jumpstart::Surface;
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
}

impl Whstlrs {
    fn new(mut context: Context, surface: Surface) -> Self {
        context.resize();
        context.gpu.submit();

        Self { context, surface }
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

                //self.update(delta);
                //self.render();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
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
