mod create;

use core::ptr::NonNull;

pub use create::{VulkanBackendParams, VulkanVersion};

use crate::backends::vulkan::{fn_table::FnTable, types};

pub struct VulkanBackend<'a> {
    instance: types::Instance,
    alloc: Option<NonNull<types::AllocationCallbacks<'a, ()>>>,
}

impl<'a> crate::backends::Backend for VulkanBackend<'a> {}

impl<'a> Drop for VulkanBackend<'a> {
    fn drop(&mut self) {
        let fns = FnTable::global().unwrap();

        unsafe {
            (fns.fn_destroy_instance)(self.instance, self.alloc);
        }
    }
}
