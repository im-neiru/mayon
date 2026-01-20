use core::{
    ffi::{CStr, c_char},
    fmt,
};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct ExtensionName(*const c_char);

impl ExtensionName {
    pub const SURFACE: Self = Self::new(c"VK_KHR_surface");
    pub const WIN32_SURFACE: Self = Self::new(c"VK_KHR_win32_surface");
    pub const XCB_SURFACE: Self = Self::new(c"VK_KHR_xcb_surface");
    pub const XLIB_SURFACE: Self = Self::new(c"VK_KHR_xlib_surface");
    pub const WAYLAND_SURFACE: Self = Self::new(c"VK_KHR_wayland_surface");
    pub const ANDROID_SURFACE: Self = Self::new(c"VK_KHR_android_surface");
    #[allow(unused)]
    pub const IOS_SURFACE: Self = Self::new(c"VK_MVK_ios_surface");
    pub const MACOS_SURFACE: Self = Self::new(c"VK_MVK_macos_surface");

    /// Creates an `ExtensionName` from a static C string by storing its raw pointer.
    ///
    /// The provided `CStr` must have a `'static` lifetime and remain valid for as long as the
    /// resulting `ExtensionName` is used.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ffi::CStr;
    /// let s = CStr::from_bytes_with_nul(b"VK_KHR_surface\0").unwrap();
    /// let ext = ExtensionName::new(s);
    /// ```
    #[inline]
    pub(crate) const fn new(value: &'static CStr) -> Self {
        Self(value.as_ptr())
    }
}

impl fmt::Debug for ExtensionName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c_str = unsafe { CStr::from_ptr(self.0) };
        write!(f, "{}", c_str.to_str().unwrap())
    }
}
