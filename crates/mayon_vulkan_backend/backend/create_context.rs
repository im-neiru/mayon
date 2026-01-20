use allocator::Allocator;
use mayon_core::{CreateContextFromRwh, logger::Logger};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::{Error, VulkanContext, fn_table::FnTable};

impl<L, A> CreateContextFromRwh<L, A> for crate::VulkanBackend<'_, L, A>
where
    L: Logger,
    A: Allocator,
{
    type Error = Error;
    fn create_context_from_rwh<H>(
        instance: &mut mayon_core::InstanceRef<Self, L, A>,
        handle: &H,
    ) -> Result<
        Self::Context,
        mayon_core::CreateContextError<<Self::Error as mayon_core::BaseError>::ErrorKind>,
    >
    where
        H: HasDisplayHandle + HasWindowHandle,
    {
        let fns = FnTable::global().unwrap();

        Ok(VulkanContext {
            instance: instance.clone(),
        })
    }
}
