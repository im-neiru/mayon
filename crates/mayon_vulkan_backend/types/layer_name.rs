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
        let mut left = self.0.as_ptr().cast_const();
        let mut right = other.0.as_ptr().cast_const();

        // Skip pointer optimization in tests to force manual string comparison.
        #[cfg(not(test))]
        if core::ptr::eq(left, right) {
            return true;
        }

        unsafe {
            loop {
                if left.read() != right.read() {
                    return false;
                }

                if left.read() == 0 {
                    return true;
                }

                left = left.add(1);
                right = right.add(1);
            }
        }
    }
}

impl<const SIZE: usize> PartialEq<[c_char; SIZE]> for LayerName {
    fn eq(&self, other: &[c_char; SIZE]) -> bool {
        let mut left = self.0.as_ptr().cast_const();
        let mut right = other.as_ptr();

        // Skip pointer optimization in tests to force manual string comparison.
        #[cfg(not(test))]
        if core::ptr::eq(left, right) {
            return true;
        }

        unsafe {
            for _ in 0..SIZE {
                if left.read() != right.read() {
                    return false;
                }

                if left.read() == 0 {
                    return true;
                }

                left = left.add(1);
                right = right.add(1);
            }

            left.read() == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_name_eq() {
        let name1 = LayerName::new(c"VK_LAYER_KHRONOS_validation");
        let name2 = LayerName::new(c"VK_LAYER_KHRONOS_validation");
        let name3 = LayerName::new(c"VK_LAYER_LUNARG_api_dump");
        let name4 = LayerName::new(c"VK_LAYER_KHRONOS");

        assert_eq!(name1, name2, "Different pointers, same content");
        assert_eq!(name1, name1, "Same pointer (identity)");
        assert_eq!(name1, LayerName::VALIDATION, "Comparison with constant");
        assert_ne!(name1, name3, "Different content");
        assert_ne!(name1, name4, "Prefix should not be equal");
        assert_ne!(
            name4, name1,
            "Shorter string should not be equal to longer string"
        );
    }

    #[test]
    fn test_layer_name_array_eq() {
        let name = LayerName::new(c"VK_LAYER_KHRONOS_validation");

        let mut array = [0 as c_char; 256];
        let bytes = b"VK_LAYER_KHRONOS_validation\0";
        for (i, &b) in bytes.iter().enumerate() {
            array[i] = b as c_char;
        }

        assert_eq!(name, array, "Correct content in array");

        let mut prefix_array = [0 as c_char; 256];
        let prefix_bytes = b"VK_LAYER_KHRONOS\0";
        for (i, &b) in prefix_bytes.iter().enumerate() {
            prefix_array[i] = b as c_char;
        }
        assert_ne!(
            name, prefix_array,
            "Array with prefix content should not match"
        );

        let mut longer_array = [0 as c_char; 256];
        let longer_bytes = b"VK_LAYER_KHRONOS_validation_extra\0";

        for (i, &b) in longer_bytes.iter().enumerate() {
            longer_array[i] = b as c_char;
        }

        assert_ne!(
            name, longer_array,
            "Array with longer content should not match"
        );
    }
}
