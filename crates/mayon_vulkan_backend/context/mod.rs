use allocator::Allocator;
use mayon_core::{Context, InstanceRef, logger::Logger};

use crate::VulkanBackend;

pub struct VulkanContext<'a, L, A>
where
    L: Logger,
    A: Allocator,
{
    pub(crate) instance: InstanceRef<VulkanBackend<'a, L, A>, L, A>,
}

impl<'a, L, A> Context for VulkanContext<'a, L, A>
where
    L: Logger,
    A: Allocator,
{
    // TODO: implement stuff
}
