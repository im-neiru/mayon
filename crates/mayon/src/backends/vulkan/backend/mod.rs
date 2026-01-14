mod create;

use core::alloc::Allocator;
use std::alloc::Global;

pub use create::{VulkanBackendParams, VulkanVersion};

use crate::backends::vulkan::{fn_table::FnTable, types};

pub struct VulkanBackend<'a, A = Global>
where
    A: Allocator + 'static,
{
    instance: types::Instance,
    alloc: types::AllocationCallbacks<'a, A>,
}

impl<'a, A> crate::backends::Backend for VulkanBackend<'a, A> where A: core::alloc::Allocator {}

impl<'a, A> Drop for VulkanBackend<'a, A>
where
    A: Allocator,
{
    fn drop(&mut self) {
        let fns = FnTable::global().unwrap();

        unsafe {
            (fns.fn_destroy_instance)(self.instance, self.alloc.alloc_ref());
        }
    }
}
