mod create;

pub use create::{VulkanBackendParams, VulkanVersion};

use crate::backends::vulkan::{fn_table::FnTable, types};

pub struct VulkanBackend {
    instance: types::Instance,
}

impl crate::backends::Backend for VulkanBackend {}
