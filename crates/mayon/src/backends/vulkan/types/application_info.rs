use core::{
    ffi::{CStr, c_char, c_void},
    marker::PhantomData,
};
use std::ptr::null;

use crate::backends::vulkan::{VulkanBackendParams, VulkanVersion};

use super::StructureType;

#[repr(C)]
pub(in crate::backends::vulkan) struct ApplicationInfo<'a> {
    pub struct_type: StructureType,
    pub next: *const c_void,
    pub application_name: *const c_char,
    pub application_version: u32,
    pub engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
    pub _marker: PhantomData<&'a ()>,
}

const VULKAN_API_VERSION: u32 = VulkanVersion::new(1, 2, 0).raw();

impl ApplicationInfo<'_> {
    pub(crate) fn new(
        VulkanBackendParams {
            application_name,
            application_version,
            engine_name,
            engine_version,
            target_platform: _,
        }: VulkanBackendParams,
    ) -> Self {
        Self {
            struct_type: StructureType::ApplicationInfo,
            next: null(),
            application_name: application_name.map(CStr::as_ptr).unwrap_or_else(null),
            application_version: application_version.raw(),
            engine_name: engine_name.map(CStr::as_ptr).unwrap_or_else(null),
            engine_version: engine_version.raw(),
            api_version: VULKAN_API_VERSION,
            _marker: PhantomData,
        }
    }
}
