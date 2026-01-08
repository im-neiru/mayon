use core::ffi::{CStr, c_char};

#[inline(always)]
pub(crate) const fn ptr_to_op_cstr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}
