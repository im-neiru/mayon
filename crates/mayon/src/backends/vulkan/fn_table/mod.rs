use core::mem::MaybeUninit;

use std::sync::OnceLock;

mod loader;

use libloading::Library;

use crate::backends::vulkan::{
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

static FN_TABLE: OnceLock<FnTable> = OnceLock::new();

impl FnTable {
    pub(in crate::backends) fn global() -> super::Result<&'static Self> {
        FN_TABLE.get_or_try_init(Self::new)
    }

    fn new() -> super::Result<Self> {
        match unsafe { loader::vulkan_lib() } {
            Ok(library) => Ok(Self {
                fn_create_instance: unsafe { *library.get("vkCreateInstance").unwrap() },
                fn_destroy_instance: unsafe { *library.get("vkDestroyInstance").unwrap() },

                library: Some(library),
            }),
            Err(err) => {
                #[cfg(debug_assertions)]
                eprintln!("Vulkan load error: {err}");

                ErrorKind::VulkanLoad.into_result()
            }
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
