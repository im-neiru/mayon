mod create;
mod create_context;

use allocator::{Allocator, System};
use mayon_core::logger::Logger;

pub use create::{VulkanBackendParams, VulkanVersion};

use crate::{fn_table::FnTable, types};

pub struct VulkanBackend<'a, L, A = System>
where
    L: Logger,
    A: Allocator,
{
    instance: types::Instance,
    alloc: types::AllocationCallbacks<'a, A>,
    _marker: std::marker::PhantomData<L>,
}

impl<'a, L, A> VulkanBackend<'a, L, A>
where
    L: Logger,
    A: Allocator,
{
    #[inline(always)]
    pub(crate) fn instance(&self) -> types::Instance {
        self.instance
    }

    #[inline(always)]
    pub(crate) unsafe fn allocator(&self) -> types::AllocationCallbacksRef<'a> {
        unsafe { self.alloc.alloc_ref() }
    }
}

impl<'a, L, A> mayon_core::Backend for VulkanBackend<'a, L, A>
where
    L: Logger,
    A: Allocator,
{
    type Context = crate::VulkanContext;
}

impl<'a, L, A> Drop for VulkanBackend<'a, L, A>
where
    L: Logger,
    A: Allocator,
{
    fn drop(&mut self) {
        let fns = FnTable::global().unwrap();

        unsafe {
            fns.destroy_instance(self.instance, self.alloc.alloc_ref());
        }
    }
}
