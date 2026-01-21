use core::{ffi::c_void, marker::PhantomData, ptr::NonNull};

use raw_window_handle::{WaylandDisplayHandle, WaylandWindowHandle};

use super::StructureType;

#[repr(C)]
pub struct WaylandSurfaceCreateInfo<'a> {
    pub structure_type: StructureType,
    pub next: Option<NonNull<c_void>>,
    pub flags: WaylandSurfaceCreateFlags,
    pub display: NonNull<c_void>,
    pub surface: NonNull<c_void>,
    pub _marker: PhantomData<&'a ()>,
}

impl WaylandSurfaceCreateInfo<'_> {
    /// Creates a `WaylandSurfaceCreateInfo` value from a `WaylandDisplayHandle` and `WaylandWindowHandle`.
    #[inline]
    pub(crate) const fn from_handle(
        display_handle: &WaylandDisplayHandle,
        window_handle: &WaylandWindowHandle,
    ) -> Self {
        Self {
            structure_type: StructureType::WaylandSurfaceCreateInfoKhr,
            next: None,
            flags: super::WaylandSurfaceCreateFlags::EMPTY, // super is is needed to silence error on the parent module
            display: display_handle.display,
            surface: window_handle.surface,
            _marker: PhantomData,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct WaylandSurfaceCreateFlags(pub(crate) u32);

impl WaylandSurfaceCreateFlags {
    pub const EMPTY: Self = Self(0);
}
