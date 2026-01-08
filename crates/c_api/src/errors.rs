use core::{cell::RefCell, ffi::CStr};

use crate::fallible_result::FallibleResult;

thread_local! {
    static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
}

pub(crate) enum Error {
    NullPointerArg {
        name: &'static CStr,
    },
    FailedBackendLoad {
        name: &'static CStr,
    },
    VulkanError {
        function_name: &'static str,
        return_code: i32,
    },
}

#[inline]
pub(crate) fn set_ok() -> FallibleResult {
    LAST_ERROR.set(None);

    FallibleResult::Ok
}

#[inline]
pub(crate) fn set_null_pointer_arg(name: &'static CStr) -> FallibleResult {
    LAST_ERROR.set(Some(Error::NullPointerArg { name }));

    FallibleResult::NullPointerParam
}

#[inline]
pub(crate) fn set_vulkan_error(error: mayon::backends::vulkan::Error) -> FallibleResult {
    match error.kind() {
        mayon::backends::vulkan::ErrorKind::VulkanLoad => {
            LAST_ERROR.set(Some(Error::FailedBackendLoad { name: c"Vulkan" }));

            FallibleResult::BackendLoadFailed
        }
        mayon::backends::vulkan::ErrorKind::VulkanFunctionError {
            function_name,
            code,
        } => {
            LAST_ERROR.set(Some(Error::VulkanError {
                function_name,
                return_code: code as i32,
            }));

            FallibleResult::VulkanFunctionError
        }
    }
}
