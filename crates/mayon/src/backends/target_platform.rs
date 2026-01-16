use bitflags::bitflags;
use raw_window_handle::RawDisplayHandle;

bitflags! {
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TargetPlatform: u16 {
        const WAYLAND     = 0b0000_0001;
        const XCB         = 0b0000_0010;
        const XLIB        = 0b0000_0100;
        const WIN32       = 0b0000_1000;
        const ANDROID     = 0b0001_0000;
        const METAL       = 0b0010_0000;
        const HEADLESS    = 0b0100_0000;
    }
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
#[error("Unsupported target windowing platform")]
pub struct UnsupportedPlatformError;

impl TargetPlatform {
    #[inline]
    pub(in crate::backends) const fn from_raw_display_handle(
        handle: RawDisplayHandle,
        with_headless: bool,
    ) -> Result<Self, UnsupportedPlatformError> {
        let window = match handle {
            RawDisplayHandle::Wayland(_) => Self::WAYLAND,
            RawDisplayHandle::Xcb(_) => Self::XCB,
            RawDisplayHandle::Xlib(_) => Self::XLIB,
            RawDisplayHandle::Windows(_) => Self::WIN32,
            RawDisplayHandle::Android(_) => Self::ANDROID,
            RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => Self::METAL,

            _ => return Err(UnsupportedPlatformError),
        };

        Ok(if with_headless {
            window.union(Self::HEADLESS)
        } else {
            window
        })
    }
}
