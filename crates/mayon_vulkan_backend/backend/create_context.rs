use allocator::Allocator;
use mayon_core::{CreateContextFromRwh, logger::Logger};

use crate::{VulkanContext, fn_table::FnTable};

impl<L, A> CreateContextFromRwh<L, A> for crate::VulkanBackend<'_, L, A>
where
    L: Logger,
    A: Allocator,
{
    fn create_context_from_rwh<H>(
        &self,
        instance: &mut mayon_core::InstanceRef<Self, L, A>,
        handle: H,
    ) -> Self::Context
    where
        H: raw_window_handle::HasDisplayHandle + raw_window_handle::HasWindowHandle,
    {
        let fns = FnTable::global().unwrap();

        VulkanContext {
            instance: instance.clone(),
        }
    }
}
