use allocator::Allocator;
use mayon_core::{ContextHandler, DestroyContext, InstanceRef, logger::Logger};

use crate::{VulkanBackend, fn_table::FnTable, types::Surface};

pub struct VulkanContext {
    pub(crate) surface: Surface,
}

impl ContextHandler for VulkanContext {}

impl<'a, L, A> DestroyContext<VulkanBackend<'a, L, A>, L, A> for VulkanContext
where
    L: Logger,
    A: Allocator,
{
    fn destroy(&self, instance: &InstanceRef<VulkanBackend<'a, L, A>, L, A>) {
        unsafe {
            let fns = FnTable::global().unwrap();

            let vk_instance = instance.backend().instance();
            let vk_allocator = unsafe { instance.backend().allocator() };

            fns.destroy_surface(vk_instance, self.surface, vk_allocator);
        }
    }
}
