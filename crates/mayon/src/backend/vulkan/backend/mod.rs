mod create;

pub use create::VulkanBackendParams;

use crate::backend::vulkan::fn_table::FnTable;

pub struct VulkanBackend {}

impl crate::backend::Backend for VulkanBackend {}
