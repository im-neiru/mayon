use core::mem::MaybeUninit;

use once_cell::sync::OnceCell;

mod loader;

use libloading::Library;

use crate::{
    ErrorKind,
    types::{AllocationCallbacksRef, Instance, InstanceCreateInfo, VkResult},
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
                fn_create_instance: unsafe { *library.get("vkCreateInstance").unwrap() },
                fn_destroy_instance: unsafe { *library.get("vkDestroyInstance").unwrap() },

                library: Some(library),
            }),
            Err(_) => ErrorKind::VulkanLoad.into_result(),
        }
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
