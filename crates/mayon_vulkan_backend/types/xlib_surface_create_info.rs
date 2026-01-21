use core::{ffi::c_void, marker::PhantomData, ptr::NonNull};

use raw_window_handle::{XlibDisplayHandle, XlibWindowHandle};

use super::StructureType;

#[repr(C)]
pub struct XlibSurfaceCreateInfo<'a> {
    pub structure_type: StructureType,
    pub next: Option<NonNull<c_void>>,
    pub flags: XlibSurfaceCreateFlags,
    pub display: NonNull<c_void>,
    pub window: core::ffi::c_ulong,
    pub _marker: PhantomData<&'a ()>,
}

impl XlibSurfaceCreateInfo<'_> {
    /// Creates a `XlibSurfaceCreateInfo` value from a `XlibDisplayHandle` and `XlibWindowHandle`.
    #[inline]
    pub(crate) const fn from_handle(
        display_handle: &XlibDisplayHandle,
        window_handle: &XlibWindowHandle,
    ) -> Self {
        // SAFETY: The display handle is expected to contain a valid display pointer.
        // We assume the caller ensures this remains valid.
        let display = match display_handle.display {
            Some(dpy) => dpy,
            None => panic!("XlibDisplayHandle must contain a valid display pointer"),
        };

        Self {
            structure_type: StructureType::XlibSurfaceCreateInfoKhr,
            next: None,
            flags: super::XlibSurfaceCreateFlags::EMPTY,
            display,
            window: window_handle.window,
            _marker: PhantomData,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct XlibSurfaceCreateFlags(pub(crate) u32);

impl XlibSurfaceCreateFlags {
    pub const EMPTY: Self = Self(0);
}
