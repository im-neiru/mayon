use allocator::Allocator;
use mayon_core::{
    CreateContextError, CreateContextErrorKind, CreateContextFromRwh, logger::Logger,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

use crate::{VulkanContext, VulkanError, fn_table::FnTable, types::Win32SurfaceCreateInfo};

impl<L, A> CreateContextFromRwh<L, A> for crate::VulkanBackend<'_, L, A>
where
    L: Logger,
    A: Allocator,
{
    type Error = VulkanError;
    fn create_context_from_rwh<H>(
        instance: &mayon_core::InstanceRef<Self, L, A>,
        handle: &H,
    ) -> Result<Self::Context, CreateContextError<<Self::Error as mayon_core::BaseError>::ErrorKind>>
    where
        H: HasDisplayHandle + HasWindowHandle,
    {
        let fns = FnTable::global()?;

        let (vk_instance, alloc_callbacks) = unsafe {
            let backend = instance.backend();

            (backend.instance, backend.alloc.alloc_ref())
        };

        let display_handle = handle.display_handle().map(|handle| handle.as_raw());
        let window_handle = handle.window_handle().map(|handle| handle.as_raw());

        #[allow(clippy::let_unit_value)]
        let surface = match (display_handle, window_handle) {
            (Ok(RawDisplayHandle::Windows(_)), Ok(RawWindowHandle::Win32(handle))) => unsafe {
                fns.create_win32_surface(
                    vk_instance,
                    &Win32SurfaceCreateInfo::from_handle(&handle),
                    alloc_callbacks,
                )?
            },

            // TODO: Add support for other platforms
            _ => return CreateContextErrorKind::UnsupportedPlatform.into_result(),
        };

        Ok(VulkanContext { surface })
    }
}
