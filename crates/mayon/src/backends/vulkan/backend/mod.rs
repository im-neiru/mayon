mod create;

pub use create::VulkanBackendParams;

use crate::backends::vulkan::fn_table::FnTable;

pub struct VulkanBackend {}

impl crate::backends::Backend for VulkanBackend {}
