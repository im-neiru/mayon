mod create;

use allocator::{Allocator, System};

pub use create::{VulkanBackendParams, VulkanVersion};

use crate::{fn_table::FnTable, types};

pub struct VulkanBackend<'a, A = System>
where
    A: Allocator + 'static,
{
    instance: types::Instance,
    alloc: types::AllocationCallbacks<'a, A>,
}

impl<'a, A> mayon_core::Backend for VulkanBackend<'a, A>
where
    A: Allocator,
{
    type Context = crate::VulkanContext;
}

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
