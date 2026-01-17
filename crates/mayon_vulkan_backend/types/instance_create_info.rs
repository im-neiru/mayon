use core::{
    ffi::{c_char, c_void},
    marker::PhantomData,
};
use std::ptr::null;

use super::{ApplicationInfo, ExtensionName, StructureType};

#[repr(C)]
pub(crate) struct InstanceCreateInfo<'a> {
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
    /// Creates an FFI-compatible InstanceCreateInfo tied to lifetime `'a`.
    ///
    /// Constructs an InstanceCreateInfo populated with the provided application info,
    /// array of C string pointers for layer names, and array of `ExtensionName` values.
    /// The returned struct is suitable for passing to Vulkan FFI calls.
    ///
    /// # Parameters
    ///
    /// - `application_info`: Reference to the application information to attach to the create info.
    /// - `layer_names`: Slice of C string pointers (`*const c_char`) representing enabled layer names (null-terminated).
    /// - `extension_names`: Slice of `ExtensionName` values representing enabled extensions.
    ///
    /// # Returns
    ///
    /// An initialized `InstanceCreateInfo<'a>` with counts and raw pointers set from the provided slices.
    ///
    /// # Examples
    ///
    /// ```
    /// let app_info = ApplicationInfo::default();
    /// let layers: &[*const std::os::raw::c_char] = &[];
    /// let extensions: &[ExtensionName] = &[];
    /// let create_info = InstanceCreateInfo::new(&app_info, layers, extensions);
    /// assert_eq!(create_info.enabled_layer_count, 0);
    /// assert_eq!(create_info.enabled_extension_count, 0);
    /// ```
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