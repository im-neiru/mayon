use core::{
    ffi::{CStr, c_char},
    fmt,
    ptr::NonNull,
};

#[repr(transparent)]
#[derive(Clone, Copy, Eq)]
pub(crate) struct LayerName(NonNull<c_char>);

impl LayerName {
    pub(crate) const VALIDATION: Self = Self::new(c"VK_LAYER_KHRONOS_validation");

    #[inline]
    pub(crate) const fn new(value: &'static CStr) -> Self {
        Self(unsafe { NonNull::new_unchecked(value.as_ptr().cast_mut() as *mut c_char) })
    }
}

impl fmt::Debug for LayerName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c_str = unsafe { CStr::from_ptr(self.0.as_ptr()) };
        write!(f, "{}", c_str.to_string_lossy())
    }
}

impl PartialEq for LayerName {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let mut left = self.0.as_ptr();
        let mut right = other.0.as_ptr();

        if left == right {
            return true;
        }

        unsafe {
            loop {
                if left.read() == 0 {
                    return true;
                }

                if left.read() != right.read() {
                    return false;
                }

                left = left.add(1);
                right = right.add(1);
            }
        }
    }
}
