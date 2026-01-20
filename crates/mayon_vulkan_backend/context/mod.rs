use allocator::Allocator;
use mayon_core::{ContextHandler, InstanceRef, logger::Logger};

use crate::VulkanBackend;

pub struct VulkanContext<'a, L, A>
where
    L: Logger,
    A: Allocator,
{
    pub(crate) instance: InstanceRef<VulkanBackend<'a, L, A>, L, A>,
}

impl<'a, L, A> ContextHandler for VulkanContext<'a, L, A>
where
    L: Logger,
    A: Allocator,
{
    // TODO: implement stuff
}
