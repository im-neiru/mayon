#![feature(allocator_api)]

mod allocator;
mod conversions;
mod errors;
mod fallible_result;

use core::ffi::c_char;
use fallible_result::MynFallibleResult;

use mayon::{
    Instance,
    backends::vulkan::{VulkanBackend, VulkanBackendParams},
    logger::DefaultLogger,
};

use crate::allocator::MynCustomAllocator;

/// @brief Vulkan backend initialization parameters.
///
/// @note All pointer fields are borrowed for the duration of the call.
#[repr(C)]
pub struct MynVkBackendParams {
    /// @brief Optional null-terminated UTF-8 application name.
    pub application_name: *const c_char,

    /// @brief Application version.
    pub application_version: MynVkVersion,

    /// @brief Optional null-terminated UTF-8 engine name.
    pub engine_name: *const c_char,

    /// @brief Engine version.
    pub engine_version: MynVkVersion,
}

/// @brief Vulkan version structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MynVkVersion {
    /// @brief Major version number (e.g., 1 in Vulkan 1.3.0).
    pub major: u32,
    /// @brief Minor version number (e.g., 3 in Vulkan 1.3.0).
    pub minor: u32,
    /// @brief Patch version number (e.g., 0 in Vulkan 1.3.0).
    pub patch: u32,
}

/// @brief Opaque Mayon instance handle.
///
/// @note Instances are reference-counted internally.
#[repr(C)]
pub struct MynInstance(usize);

/// @brief Creates a new Mayon instance using the Vulkan API as backend.
///
/// @param params Pointer to a \c MynVkBackendParams structure. Must not be \c NULL.
/// @param allocator use to set a custom allocator.
/// @param out_instance Pointer to storage that will receive the created Instance. Must not be \c NULL.
///
/// @return \c MAYON_RESULT_OK on success.
/// @return A non-zero \c MynFallibleResult error code on failure.
/// @return \c MAYON_RESULT_NULL_ARG if \p params or \p out_instance is \c NULL.
///
/// @par Behavior
/// On success, a valid Instance handle is written to \p out_instance.
/// On failure, *\p out_instance remains unchanged and an error message is stored
/// (retrievable via \c mayon_last_error_message()).
///
/// @par Requirements
/// - \p params must point to a valid \c MynVkBackendParams structure.
/// - \p out_instance must point to writable, properly aligned memory.
/// - All string pointers within \p params must be valid null-terminated UTF-8 C strings.
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn mayon_new_instance_on_vulkan(
    params: *const MynVkBackendParams,
    allocator: *const allocator::MynCustomAllocator,
    out_instance: *mut MynInstance,
) -> MynFallibleResult {
    if out_instance.is_null() {
        return errors::set_null_pointer_arg(c"out_instance");
    }

    let Some(params) = params.as_ref() else {
        return errors::set_null_pointer_arg(c"params");
    };

    let rust_params = VulkanBackendParams {
        application_name: conversions::ptr_to_op_cstr(params.application_name),
        application_version: params.application_version.into(),
        engine_name: conversions::ptr_to_op_cstr(params.engine_name),
        engine_version: params.engine_version.into(),
    };

    match Instance::new_in::<'static, VulkanBackend<'_, allocator::MynCustomAllocator>>(
        rust_params,
        if allocator.is_null() {
            allocator::MynCustomAllocator::DEFAULT
        } else {
            *allocator
        },
        DefaultLogger,
    ) {
        Ok(instance) => {
            out_instance.write(instance.into());

            errors::set_ok()
        }
        Err(err) => errors::set_vulkan_error(err),
    }
}

/// @brief Releases a Mayon instance.
///
/// @param instance Pointer to the Mayon instance to release.
///
/// @note
///
/// \par Safety
///
/// - Instances are internally reference-counted. Releasing the same instance
///   multiple times may cause unintended deallocation once the reference count
///   reaches zero.
///
/// - Passing a null pointer has no effect.
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn mayon_drop_instance(instance: *mut MynInstance) {
    let Some(instance) = instance
        .as_mut()
        .map(MynInstance::inner_mut::<MynCustomAllocator, DefaultLogger>)
    else {
        return;
    };

    core::ptr::drop_in_place(instance);
}

/// @brief Returns the last error message for the calling thread.
///
/// @returns Pointer to a null-terminated UTF-8 string describing the last error.
/// @returns NULL if no error is currently set.
///
/// @par Lifetime and ownership:
/// - The returned pointer must NOT be freed.
/// - The pointer remains valid until the next error is set on the same thread.
///
/// @par Threading:
/// - Error messages are stored per-thread.
/// - Calling this function does not affect other threads.
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn mayon_last_error_message() -> *const c_char {
    errors::get_message()
}
