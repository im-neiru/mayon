use core::{
    cell::RefCell,
    ffi::{CStr, c_char},
};

use std::ptr::null;

use mayon::{
    BaseError, CreateBackendError, CreateBackendErrorKind, backends::vulkan::VulkanErrorKind,
};

use crate::fallible_result::MynFallibleResult;

thread_local! {
    static LAST_ERROR: RefCell<Option<Error>> = const { RefCell::new(None) };
    static ERROR_MSG_BUFFER: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub(crate) enum Error {
    NullArg {
        name: &'static CStr,
    },
    InstanceAllocation,
    UnsupportedTargetPlatform,
    FailedBackendLoad {
        name: &'static CStr,
    },
    VulkanFunction {
        function_name: &'static str,
        return_code: i32,
    },
}

#[inline]
pub(crate) fn set_ok() -> MynFallibleResult {
    LAST_ERROR.set(None);

    MynFallibleResult::MAYON_RESULT_OK
}

/// Records that a null pointer was passed for a named argument in the current thread's error state.
///
/// # Parameters
///
/// - `name`: The C string identifying which argument was null (must be a static `CStr`).
///
/// # Returns
///
/// `MynFallibleResult::MAYON_RESULT_NULL_ARG`.
#[inline]
pub(crate) fn set_null_pointer_arg(name: &'static CStr) -> MynFallibleResult {
    LAST_ERROR.set(Some(Error::NullArg { name }));

    MynFallibleResult::MAYON_RESULT_NULL_ARG
}

/// Map a Vulkan backend creation error into the thread-local error state and return the corresponding MynFallibleResult.
///
/// Sets LAST_ERROR to a variant describing the provided `error` and returns the matching `MynFallibleResult` code:
/// - Records `UnsupportedTargetPlatform` and returns `MAYON_RESULT_UNSUPPORTED_PLATFORM_ERROR`.
/// - Records `FailedBackendLoad { name: "Vulkan" }` and returns `MAYON_RESULT_BACKEND_LOAD_ERROR`.
/// - Records `VulkanFunction { function_name, return_code }` and returns `MAYON_RESULT_VULKAN_LOAD_ERROR`.
///
/// # Returns
///
/// The `MynFallibleResult` value that corresponds to the recorded error.
#[inline]
pub(crate) fn set_vulkan_error(error: CreateBackendError<VulkanErrorKind>) -> MynFallibleResult {
    match error.kind() {
        CreateBackendErrorKind::UnsupportedTargetPlatform => {
            LAST_ERROR.set(Some(Error::UnsupportedTargetPlatform));

            MynFallibleResult::MAYON_RESULT_UNSUPPORTED_PLATFORM_ERROR
        }
        CreateBackendErrorKind::BackendInternal(VulkanErrorKind::LibraryLoad) => {
            LAST_ERROR.set(Some(Error::FailedBackendLoad { name: c"Vulkan" }));

            MynFallibleResult::MAYON_RESULT_BACKEND_LOAD_ERROR
        }
        CreateBackendErrorKind::BackendInternal(VulkanErrorKind::FunctionLoadFailed { name }) => {
            LAST_ERROR.set(Some(Error::VulkanFunction {
                function_name: name.into(),
                return_code: 0,
            }));

            MynFallibleResult::MAYON_RESULT_VULKAN_LOAD_ERROR
        }
        CreateBackendErrorKind::BackendInternal(VulkanErrorKind::FunctionReturn { name, code }) => {
            LAST_ERROR.set(Some(Error::VulkanFunction {
                function_name: name.into(),
                return_code: code as i32,
            }));

            MynFallibleResult::MAYON_RESULT_VULKAN_LOAD_ERROR
        }
        CreateBackendErrorKind::AllocationFailed => {
            LAST_ERROR.set(Some(Error::InstanceAllocation));

            MynFallibleResult::MAYON_RESULT_BACKEND_ALLOCATION
        }
    }
}

/// Fetches the last thread-local error message and returns it as a C string pointer.
///
/// If an error has been recorded for the current thread, the function stores a C-compatible
/// NUL-terminated representation of the message in an internal thread-local buffer and
/// returns a pointer to that buffer. If no error is recorded, the function returns null.
///
/// # Returns
///
/// `*const c_char` pointer to a NUL-terminated C string with the last error message, or `null`
/// if no error is set.
#[inline(always)]
pub(crate) fn get_message() -> *const c_char {
    LAST_ERROR.with_borrow(|err| {
        let Some(err) = err else {
            return null();
        };

        match err {
            Error::NullArg { name } => store_message(format!("Null pointer argument: {:?}", name)),
            Error::InstanceAllocation => store_message("Instance allocation failed".to_string()),
            Error::UnsupportedTargetPlatform => {
                store_message("UnsupportedPlatformError".to_string())
            }
            Error::FailedBackendLoad { name } => {
                store_message(format!("Failed to load {:?} backend", name))
            }
            Error::VulkanFunction {
                function_name,
                return_code,
            } => store_message(format!(
                "Vulkan Error: {function_name} return {return_code}"
            )),
        }
    })
}

fn store_message(mut message: String) -> *const c_char {
    message.push('0');

    ERROR_MSG_BUFFER.set(Some(message));

    ERROR_MSG_BUFFER.with_borrow(|message| {
        let Some(message) = message else {
            return null();
        };

        message.as_ptr().cast()
    })
}
