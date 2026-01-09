use core::{
    cell::RefCell,
    ffi::{CStr, c_char},
};

use std::ptr::null;

use crate::fallible_result::MynFallibleResult;

thread_local! {
    static LAST_ERROR: RefCell<Option<Error>> = const { RefCell::new(None) };
    static ERROR_MSG_BUFFER: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub(crate) enum Error {
    NullArg {
        name: &'static CStr,
    },
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

#[inline]
pub(crate) fn set_null_pointer_arg(name: &'static CStr) -> MynFallibleResult {
    LAST_ERROR.set(Some(Error::NullArg { name }));

    MynFallibleResult::MAYON_RESULT_NULL_ARG
}

#[inline]
pub(crate) fn set_vulkan_error(error: mayon::backends::vulkan::Error) -> MynFallibleResult {
    match error.kind() {
        mayon::backends::vulkan::ErrorKind::VulkanLoad => {
            LAST_ERROR.set(Some(Error::FailedBackendLoad { name: c"Vulkan" }));

            MynFallibleResult::MAYON_RESULT_BACKEND_LOAD_ERROR
        }
        mayon::backends::vulkan::ErrorKind::VulkanFunctionError {
            function_name,
            code,
        } => {
            LAST_ERROR.set(Some(Error::VulkanFunction {
                function_name,
                return_code: code as i32,
            }));

            MynFallibleResult::MAYON_RESULT_VULKAN_LOAD_ERROR
        }
    }
}

#[inline(always)]
pub(crate) fn get_message() -> *const c_char {
    LAST_ERROR.with_borrow(|err| {
        let Some(err) = err else {
            return null();
        };

        match err {
            Error::NullArg { name } => store_message(format!("Null pointer argument: {:?}", name)),
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
