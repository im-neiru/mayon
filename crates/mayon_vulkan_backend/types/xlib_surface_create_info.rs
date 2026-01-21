use core::{
    ffi::c_void,
    fmt::{Debug, Display},
    marker::PhantomData,
    ptr::NonNull,
};

use mayon_core::{CreateContextError, CreateContextErrorKind};
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
    pub(crate) const fn try_from_handle<B>(
        display_handle: &XlibDisplayHandle,
        window_handle: &XlibWindowHandle,
    ) -> Result<Self, CreateContextError<B>>
    where
        B: Copy + Clone + Debug + Display,
    {
        let Some(display) = display_handle.display else {
            return CreateContextErrorKind::<B>::HandleError.into_result();
        };

        Ok(Self {
            structure_type: StructureType::XlibSurfaceCreateInfoKhr,
            next: None,
            flags: super::XlibSurfaceCreateFlags::EMPTY,
            display,
            window: window_handle.window,
            _marker: PhantomData,
        })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct XlibSurfaceCreateFlags(pub(crate) u32);

impl XlibSurfaceCreateFlags {
    pub const EMPTY: Self = Self(0);
}
