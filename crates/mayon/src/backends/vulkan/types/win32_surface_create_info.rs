use core::{ffi::c_void, marker::PhantomData, num::NonZeroUsize, ptr::NonNull};

use super::StructureType;

#[repr(C)]
pub struct Win32SurfaceCreateInfo<'a> {
    pub structure_type: StructureType,
    pub next: NonNull<c_void>,
    pub flags: Win32SurfaceCreateFlags,
    pub hinstance: NonZeroUsize,
    pub hwnd: NonZeroUsize,
    pub _marker: PhantomData<&'a ()>,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Win32SurfaceCreateFlags(pub(crate) u32);

impl Win32SurfaceCreateFlags {
    pub const EMPTY: Self = Self(0);
}
