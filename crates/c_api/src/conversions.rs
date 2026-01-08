use core::{
    ffi::{CStr, c_char},
    mem::transmute,
};

use crate::rs;

impl From<super::VulkanVersion> for rs::VulkanVersion {
    fn from(value: super::VulkanVersion) -> Self {
        let super::VulkanVersion {
            major,
            minor,
            patch,
        } = value;

        Self {
            major,
            minor,
            patch,
        }
    }
}

impl From<rs::Instance> for super::Instance {
    fn from(value: rs::Instance) -> Self {
        unsafe { transmute(value) }
    }
}

pub(crate) fn ptr_to_op_cstr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}
