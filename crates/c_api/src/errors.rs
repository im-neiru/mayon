use core::cell::RefCell;
use core::ffi::CStr;

thread_local! {
    static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
}

pub(crate) enum Error {
    NullPointerArg {
        name: &'static CStr,
    },
    VulkanError {
        error: mayon::backends::vulkan::Error,
    },
}

pub(crate) fn set_null_pointer_arg(name: &'static CStr) {
    LAST_ERROR.set(Some(Error::NullPointerArg { name }))
}

pub(crate) fn set_vulkan_error(error: mayon::backends::vulkan::Error) {
    LAST_ERROR.set(Some(Error::VulkanError { error }))
}
