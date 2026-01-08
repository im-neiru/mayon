use core::{cell::RefCell, ffi::CStr};

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
pub(crate) fn set_null_pointer_arg(name: &'static CStr) -> i32 {
    LAST_ERROR.set(Some(Error::NullPointerArg { name }));

    -1
}

#[inline]
pub(crate) fn set_vulkan_error(error: mayon::backends::vulkan::Error) -> i32 {
    match error.kind() {
        mayon::backends::vulkan::ErrorKind::VulkanLoad => {
            LAST_ERROR.set(Some(Error::FailedBackendLoad { name: c"Vulkan" }));
        }
        mayon::backends::vulkan::ErrorKind::VulkanFunctionError {
            function_name,
            code,
        } => LAST_ERROR.set(Some(Error::VulkanError {
            function_name,
            return_code: code as i32,
        })),
    }

    -1
}
