use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

use mayon::{
    allocator::{Allocator, System},
    backends::vulkan::{Context, Instance, VulkanBackendParams},
    logger::Logger,
};

pub struct Handler<L, A = System>
where
    L: Logger,
    A: Allocator + 'static,
{
    instance: Instance<'static, L, A>,
    window_state: Option<WindowState<L, A>>,
}

#[allow(unused)]
struct WindowState<L, A = System>
where
    L: Logger,
    A: Allocator + 'static,
{
    window: Window,
    context: Context<'static, L, A>,
}

impl<L, A> Handler<L, A>
where
    L: Logger,
    A: Allocator + 'static,
{
    pub fn new(logger: L, allocator: A, event_loop: &EventLoop<()>) -> Self {
        let instance = Instance::<'static, L, A>::new_in(
            VulkanBackendParams::default()
                .with_target_from_rwh(Some(event_loop), false)
                .unwrap()
                .with_application_name(c"Mayon")
                .with_engine_name(c"Mayon Engine")
                .with_application_version((1, 0)),
            logger,
            allocator,
        )
        .unwrap();

        Self {
            instance,
            window_state: None,
        }
    }
}

impl<L, A> ApplicationHandler for Handler<L, A>
where
    L: Logger,
    A: Allocator + 'static,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window_state.is_some() {
            return;
        }

        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Mayon")
                    .with_min_inner_size(PhysicalSize::new(800, 600)),
            )
            .unwrap();

        let context = self.instance.create_context_from_rwh(&window).unwrap();

        self.window_state = Some(WindowState { window, context });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(_s) = self.window_state.as_mut() else {
            return;
        };

        #[allow(clippy::single_match)]
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }
}
