//! C-compatible Vulkan backend API for Mayon.

mod conversions;
mod errors;
mod fallible_result;

use core::ffi::c_char;

use fallible_result::FallibleResult;

mod rs {
    pub(super) use mayon::{
        Instance,
        backends::vulkan::{VulkanBackend, VulkanBackendParams, VulkanVersion},
    };
}

/// Vulkan backend initialization parameters.
///
/// All pointer fields are borrowed for the duration of the call.
#[repr(C)]
pub struct VulkanBackendParams {
    /// Optional null-terminated UTF-8 application name.
    pub application_name: *const c_char,

    /// Application version.
    pub application_version: VulkanVersion,

    /// Optional null-terminated UTF-8 engine name.
    pub engine_name: *const c_char,

    /// Engine version.
    pub engine_version: VulkanVersion,
}

/// Vulkan version structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VulkanVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

/// Opaque Mayon instance handle.
///
/// Instances are reference-counted internally.
#[repr(C)]
pub struct Instance(usize);

/// Creates a new Mayon instance using the Vulkan backend.
///
/// Returns `0` on success and writes a valid [`Instance`] to `out_instance`.
/// Returns a non-zero value on failure.
///
/// # Safety
///
/// - `param` must point to a valid [`VulkanBackendParams`].
/// - `out_instance` must point to writable, properly aligned storage for
///   an [`Instance`].
/// - If non-null, string pointers in `param` must be valid, null-terminated
///   UTF-8 strings for the duration of the call.
///
/// On failure, `out_instance` is not written.
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn mayon_new_instance_on_vulkan(
    param: *const VulkanBackendParams,
    out_instance: *mut Instance,
) -> FallibleResult {
    if out_instance.is_null() {
        return errors::set_null_pointer_arg(c"out_instance");
    }

    let Some(param) = param.as_ref() else {
        return errors::set_null_pointer_arg(c"param");
    };

    let rust_params = rs::VulkanBackendParams {
        application_name: conversions::ptr_to_op_cstr(param.application_name),
        application_version: param.application_version.into(),
        engine_name: conversions::ptr_to_op_cstr(param.engine_name),
        engine_version: param.engine_version.into(),
    };

    match rs::Instance::new::<'static, rs::VulkanBackend>(rust_params) {
        Ok(instance) => {
            out_instance.write(instance.into());

            errors::set_ok()
        }
        Err(err) => errors::set_vulkan_error(err),
    }
}

/// Releases a Mayon instance.
///
/// Passing a null pointer has no effect.
///
/// # Safety
///
/// - `instance` must be a pointer obtained from this API or null.
/// - The instance must not be released more times than it was created.
///
/// Instances are internally reference-counted. Releasing the same instance
/// multiple times may cause unintended deallocation once the reference count
/// reaches zero.
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn mayon_drop_instance(instance: *mut Instance) {
    let Some(instance) = instance.as_mut().map(core::ops::DerefMut::deref_mut) else {
        return;
    };

    core::ptr::drop_in_place(instance);
}
