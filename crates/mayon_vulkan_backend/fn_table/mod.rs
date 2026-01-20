mod function_name;
mod loader;

pub use function_name::VulkanFunctionName;

use core::mem::MaybeUninit;

use libloading::Library;
use once_cell::sync::OnceCell;

use VulkanFunctionName::*;

use crate::{
    ErrorKind,
    types::{
        AllocationCallbacksRef, Instance, InstanceCreateInfo, Surface, VkResult,
        Win32SurfaceCreateInfo,
    },
};

pub struct FnTable {
    library: Option<Library>,

    pub fn_create_instance: unsafe extern "system" fn(
        create_info: *const InstanceCreateInfo,
        allocator: AllocationCallbacksRef,
        instance: *mut MaybeUninit<Instance>,
    ) -> VkResult,

    pub fn_destroy_instance:
        unsafe extern "system" fn(instance: Instance, allocator: AllocationCallbacksRef),

    fn_create_win32_surface: Option<
        unsafe extern "system" fn(
            instance: Instance,
            create_info: *const Win32SurfaceCreateInfo,
            allocator: AllocationCallbacksRef,
            surface: *mut Surface,
        ) -> VkResult,
    >,
}

static FN_TABLE: OnceCell<FnTable> = OnceCell::new();

impl FnTable {
    /// Returns the cached global function table, initializing and caching it on first use.
    ///
    /// On success, yields a `'static` reference to the initialized `FnTable`. Returns an error if the Vulkan library cannot be loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// // Access the global function table (may initialize the table on first call).
    /// let table = crate::backends::vulkan::FnTable::global().unwrap();
    /// // Use `table` to call loaded Vulkan functions, e.g. `table.fn_create_instance`.
    /// ```
    pub(crate) fn global() -> super::Result<&'static Self> {
        FN_TABLE.get_or_try_init(Self::new)
    }

    fn new() -> super::Result<Self> {
        match unsafe { loader::vulkan_lib() } {
            Ok(library) => Ok(Self {
                fn_create_instance: unsafe { *library.get(CreateInstance.as_ref()).unwrap() },
                fn_destroy_instance: unsafe { *library.get(DestroyInstance.as_ref()).unwrap() },
                fn_create_win32_surface: None,

                library: Some(library),
            }),
            Err(_) => ErrorKind::VulkanLoad.into_result(),
        }
    }
}

impl FnTable {
    pub(crate) unsafe fn create_win32_surface(
        &mut self,
        instance: Instance,
        create_info: &Win32SurfaceCreateInfo,
        allocator: AllocationCallbacksRef,
    ) -> super::Result<Surface> {
        let Some(library) = self.library.as_ref() else {
            return ErrorKind::VulkanLoad.into_result();
        };

        let is_none = self.fn_create_win32_surface.is_none();

        if is_none {
            let Ok(function) = (unsafe { library.get(CreateWin32Surface.as_ref()) }) else {
                return ErrorKind::FunctionLoadFailed {
                    name: CreateWin32Surface,
                }
                .into_result();
            };

            self.fn_create_win32_surface = Some(*function);
        }

        let fn_create_win32_surface =
            unsafe { self.fn_create_win32_surface.as_ref().unwrap_unchecked() };

        let mut surface = MaybeUninit::<Surface>::uninit();

        let result = unsafe {
            (fn_create_win32_surface)(instance, create_info, allocator, surface.as_mut_ptr())
        };

        result.into_result(CreateWin32Surface.as_ref(), || unsafe {
            surface.assume_init()
        })
    }
}

impl Drop for FnTable {
    #[inline]
    fn drop(&mut self) {
        if let Some(library) = self.library.take() {
            library.close().unwrap();
        }
    }
}
