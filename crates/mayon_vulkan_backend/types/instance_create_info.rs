use core::{
    ffi::{c_char, c_void},
    marker::PhantomData,
};
use std::ptr::null;

use super::{ApplicationInfo, ExtensionName, StructureType};

#[repr(C)]
pub(in crate::backends::vulkan) struct InstanceCreateInfo<'a> {
    pub struct_type: StructureType,
    pub next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub application_info: *const ApplicationInfo<'a>,
    pub enabled_layer_count: u32,
    pub enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const ExtensionName,
    pub _marker: PhantomData<&'a ()>,
}

impl<'a> InstanceCreateInfo<'a> {
    pub fn new(
        application_info: &'a ApplicationInfo,
        layer_names: &'a [*const c_char],
        extension_names: &'a [ExtensionName],
    ) -> Self {
        Self {
            struct_type: StructureType::InstanceCreateInfo,
            next: null(),
            flags: InstanceCreateFlags::EMPTY,
            application_info,
            enabled_layer_count: layer_names.len() as u32,
            enabled_layer_names: layer_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
            enabled_extension_names: extension_names.as_ptr(),
            _marker: Default::default(),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct InstanceCreateFlags(pub(crate) u32);

impl InstanceCreateFlags {
    pub const EMPTY: InstanceCreateFlags = InstanceCreateFlags(0);
}
