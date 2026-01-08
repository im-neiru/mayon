use mayon::backends::vulkan::{VulkanBackend, VulkanBackendParams};

fn main() {
    let instance = mayon::Instance::new::<'static, VulkanBackend>(
        VulkanBackendParams::default()
            .with_application_name("Mayon")
            .with_engine_name("Mayon Engine"),
    )
    .unwrap();
}
