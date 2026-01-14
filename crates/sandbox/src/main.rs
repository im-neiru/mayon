use mayon::{
    backends::vulkan::{VulkanBackend, VulkanBackendParams},
    logger::DefaultLogger,
};

fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let _instance = mayon::Instance::new::<'static, VulkanBackend>(
        VulkanBackendParams::default()
            .with_application_name(c"Mayon")
            .with_engine_name(c"Mayon Engine")
            .with_application_version((1, 0)),
        DefaultLogger,
    )
    .unwrap();
}
