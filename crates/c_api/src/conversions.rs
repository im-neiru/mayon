use core::{
    ffi::{CStr, c_char},
    mem::transmute,
    ops::{Deref, DerefMut},
};

use crate::rs;

impl From<super::VulkanVersion> for rs::VulkanVersion {
    #[inline(always)]
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
    #[inline(always)]
    fn from(value: rs::Instance) -> Self {
        unsafe { transmute::<rs::Instance, Self>(value) }
    }
}

impl Deref for super::Instance {
    type Target = rs::Instance;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        let ptr: *const Self::Target = (self as *const Self).cast();

        unsafe { ptr.as_ref().unwrap_unchecked() }
    }
}

impl DerefMut for super::Instance {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr: *mut Self::Target = (self as *mut Self).cast();

        unsafe { ptr.as_mut().unwrap_unchecked() }
    }
}

#[inline(always)]
pub(crate) const fn ptr_to_op_cstr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}
