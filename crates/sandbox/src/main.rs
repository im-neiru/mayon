use mayon::backends::vulkan::{VulkanBackend, VulkanBackendParams};

fn main() {
    let _instance = mayon::Instance::new::<'static, VulkanBackend>(
        VulkanBackendParams::default()
            .with_application_name(c"Mayon")
            .with_engine_name(c"Mayon Engine")
            .with_application_version((1, 0)),
    )
    .unwrap();
}
