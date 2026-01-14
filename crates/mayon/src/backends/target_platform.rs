use raw_window_handle::RawDisplayHandle;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, strum::IntoStaticStr, strum::Display)]
pub enum TargetPlatform {
    Wayland = 1,
    Xcb = 2,
    Xlib = 3,
    Win32 = 4,
    Android = 5,
    Metal = 6,
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
#[error("Unsupported target windowing platform")]
pub struct UnsupportedPlatformError;

impl TargetPlatform {
    #[inline]
    pub(in crate::backends) const fn from_raw_display_handle(
        handle: RawDisplayHandle,
    ) -> Result<Self, UnsupportedPlatformError> {
        match handle {
            RawDisplayHandle::Wayland(_) => Ok(Self::Wayland),
            RawDisplayHandle::Xcb(_) => Ok(Self::Xcb),
            RawDisplayHandle::Xlib(_) => Ok(Self::Xlib),
            RawDisplayHandle::Windows(_) => Ok(Self::Win32),
            RawDisplayHandle::Android(_) => Ok(Self::Android),
            RawDisplayHandle::AppKit(_) => Ok(Self::Metal),
            RawDisplayHandle::UiKit(_) => Ok(Self::Metal),

            _ => Err(UnsupportedPlatformError),
        }
    }
}
