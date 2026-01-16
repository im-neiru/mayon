use core::ffi::{CStr, c_char};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub(in crate::backends::vulkan) struct ExtensionName(*const c_char);

impl ExtensionName {
    pub const VK_KHR_SURFACE: Self = Self::new(c"VK_KHR_surface");
    pub const VK_KHR_WIN32_SURFACE: Self = Self::new(c"VK_KHR_win32_surface");
    pub const VK_KHR_XCB_SURFACE: Self = Self::new(c"VK_KHR_xcb_surface");
    pub const VK_KHR_XLIB_SURFACE: Self = Self::new(c"VK_KHR_xlib_surface");
    pub const VK_KHR_WAYLAND_SURFACE: Self = Self::new(c"VK_KHR_wayland_surface");
    pub const VK_KHR_ANDROID_SURFACE: Self = Self::new(c"VK_KHR_android_surface");
    pub const VK_MVK_IOS_SURFACE: Self = Self::new(c"VK_MVK_ios_surface");
    pub const VK_MVK_MACOS_SURFACE: Self = Self::new(c"VK_MVK_macos_surface");
    pub const VK_FUCHSIA_IMAGEPIPE_SURFACE: Self = Self::new(c"VK_FUCHSIA_imagepipe_surface");

    #[inline]
    pub(in crate::backends::vulkan) const fn new(value: &'static CStr) -> Self {
        Self(value.as_ptr())
    }
}
