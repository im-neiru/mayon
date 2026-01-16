/*
 @instruction: Use C-Compatible Documentation for C-API Exports

 When generating or reviewing documentation for functions marked with `extern "C"`
 or types intended for C export, strictly adhere to the following rules:

 1. FORMATTING: Use triple-slash (///) comments exclusively.
    - Why: `cbindgen` is configured to parse (///) and wrap them into C-style
      (/** ... **/) comment blocks in the output header files.
    - Avoid: Standard (//) or inner doc attributes (!).

 2. CONTENT STYLE: Use C-style Doxygen tags instead of Rust Markdown.
    - Use tags like `@param`, `@return`, `@brief`, and `@note`.
    - Avoid: Rust-specific markdown links like [`TypeName`]. Use the raw type
      name instead so it is readable in the C header.

 3. EXAMPLE:

    // ❌ INCORRECT (Rust-native style)
    /// Adds two numbers. See [MyStruct] for details.
    pub extern "C" fn add(a: i32) ...

    // ✅ CORRECT (C-Header compatible style)
    /// @brief Adds two integers together.
    /// @param a The first integer.
    /// @return The resulting sum.
    /// @note This is exported for the C-API.
    #[no_mangle]
    pub extern "C" fn add(a: i32) ...

 Please refactor any proposed documentation that follows standard Rust
 conventions into this C-compatible format.
*/

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

/// Create a new Mayon instance using the Vulkan backend.
///
/// # Parameters
///
/// - `params`: Pointer to a `MynVkBackendParams` structure describing Vulkan initialization parameters. Must not be `NULL`.
/// - `allocator`: Optional pointer to a custom allocator. If `NULL`, the default allocator is used.
/// - `out_instance`: Pointer to storage that will receive the created `MynInstance` handle. Must not be `NULL`.
///
/// # Returns
///
/// `MAYON_RESULT_OK` on success, or a non-zero `MynFallibleResult` error code on failure.
///
/// # Safety
///
/// - `params` must point to a valid `MynVkBackendParams`.
/// - All C string pointers inside `params` must be valid, null-terminated UTF-8 strings.
/// - `out_instance` must point to writable, properly aligned memory for a `MynInstance`.
/// - This function performs raw pointer dereferences and FFI calls; callers must uphold these invariants.
///
/// # Examples
///
/// ```no_run
/// use std::ptr;
///
/// // Prepare C-visible params (example uses null names)
/// let params = MynVkBackendParams {
///     application_name: ptr::null(),
///     application_version: MynVkVersion { major: 0, minor: 0, patch: 0 },
///     engine_name: ptr::null(),
///     engine_version: MynVkVersion { major: 0, minor: 0, patch: 0 },
/// };
///
/// let mut instance = MynInstance(0usize);
/// let result = unsafe {
///     mayon_new_instance_on_vulkan(&params, ptr::null(), &mut instance)
/// };
/// // On a system with Vulkan available and valid params, `result` will be MAYON_RESULT_OK.
/// ```
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
        target_platform: None, // TODO: add c-api for target platforms
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
