use allocator::Allocator;
use mayon_core::{CreateContextFromRwh, logger::Logger};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::{VulkanContext, fn_table::FnTable};

impl<L, A> CreateContextFromRwh<L, A> for crate::VulkanBackend<'_, L, A>
where
    L: Logger,
    A: Allocator,
{
    fn create_context_from_rwh<H>(
        instance: &mut mayon_core::InstanceRef<Self, L, A>,
        handle: &H,
    ) -> Self::Context
    where
        H: HasDisplayHandle + HasWindowHandle,
    {
        let fns = FnTable::global().unwrap();

        VulkanContext {
            instance: instance.clone(),
        }
    }
}
