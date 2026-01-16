use core::ffi::{CStr, c_char};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct ExtensionName(*const c_char);

impl ExtensionName {
    pub const SURFACE: Self = Self::new(c"VK_KHR_surface");
    pub const WIN32_SURFACE: Self = Self::new(c"VK_KHR_win32_surface");
    pub const XCB_SURFACE: Self = Self::new(c"VK_KHR_xcb_surface");
    pub const XLIB_SURFACE: Self = Self::new(c"VK_KHR_xlib_surface");
    pub const WAYLAND_SURFACE: Self = Self::new(c"VK_KHR_wayland_surface");
    pub const ANDROID_SURFACE: Self = Self::new(c"VK_KHR_android_surface");
    pub const IOS_SURFACE: Self = Self::new(c"VK_MVK_ios_surface");
    pub const MACOS_SURFACE: Self = Self::new(c"VK_MVK_macos_surface");

    #[inline]
    pub(crate) const fn new(value: &'static CStr) -> Self {
        Self(value.as_ptr())
    }
}
