mod handler;

use winit::event_loop::{ControlFlow::Wait, EventLoop};

use mayon::{allocator::System, logger::DefaultLogger};

fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let mut handler = handler::Handler::new(DefaultLogger, System);

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(Wait);

    event_loop.run_app(&mut handler).unwrap();
}
