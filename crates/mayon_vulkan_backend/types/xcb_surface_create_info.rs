use core::{ffi::c_void, marker::PhantomData, num::NonZeroU32, ptr::NonNull};

use raw_window_handle::{XcbDisplayHandle, XcbWindowHandle};

use super::StructureType;

#[repr(C)]
pub struct XcbSurfaceCreateInfo<'a> {
    pub structure_type: StructureType,
    pub next: Option<NonNull<c_void>>,
    pub flags: XcbSurfaceCreateFlags,
    pub connection: Option<NonNull<c_void>>,
    pub window: NonZeroU32,
    pub _marker: PhantomData<&'a ()>,
}

impl XcbSurfaceCreateInfo<'_> {
    /// Creates a `XcbSurfaceCreateInfo` value from a `XcbDisplayHandle` and `XcbWindowHandle`.
    #[inline]
    pub(crate) const fn from_handle(
        display_handle: &XcbDisplayHandle,
        window_handle: &XcbWindowHandle,
    ) -> Self {
        Self {
            structure_type: StructureType::XcbSurfaceCreateInfoKhr,
            next: None,
            flags: super::XcbSurfaceCreateFlags::EMPTY,
            connection: display_handle.connection,
            window: window_handle.window,
            _marker: PhantomData,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct XcbSurfaceCreateFlags(pub(crate) u32);

impl XcbSurfaceCreateFlags {
    pub const EMPTY: Self = Self(0);
}
