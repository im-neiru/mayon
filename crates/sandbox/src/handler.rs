use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    window::{Window, WindowAttributes},
};

use mayon::{
    allocator::{Allocator, System},
    backends::vulkan::{Instance, VulkanBackend, VulkanBackendParams, VulkanContext},
    logger::{DefaultLogger, Logger},
};

pub struct Handler<'a, L, A = System>
where
    L: Logger,
    A: Allocator,
{
    instance: Instance<'a, L, A>,
    window_state: Option<WindowState<'a, L, A>>,
}

struct WindowState<'a, L, A = System>
where
    L: Logger,
    A: Allocator,
{
    window: Window,
    context: VulkanContext<'a, L, A>,
}

impl<'a, L, A> Handler<'a, L, A>
where
    L: Logger,
    A: Allocator,
{
    pub fn new(logger: L, allocator: A) -> Self {
        let instance = Instance::<'a, L, A>::new_in(
            VulkanBackendParams::default()
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

impl<'a, L, A> ApplicationHandler for Handler<'a, L, A>
where
    L: Logger,
    A: Allocator,
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

        let context = self.instance.create_context_from_rwh(&window);

        self.window_state = Some(WindowState { window, context });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        #[allow(clippy::single_match)]
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
