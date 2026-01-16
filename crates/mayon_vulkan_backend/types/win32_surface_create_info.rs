use core::{ffi::c_void, marker::PhantomData, num::NonZeroIsize, ptr::NonNull};

use raw_window_handle::Win32WindowHandle;

use super::StructureType;

#[repr(C)]
pub struct Win32SurfaceCreateInfo<'a> {
    pub structure_type: StructureType,
    pub next: Option<NonNull<c_void>>,
    pub flags: Win32SurfaceCreateFlags,
    pub hinstance: Option<NonZeroIsize>,
    pub hwnd: NonZeroIsize,
    pub _marker: PhantomData<&'a ()>,
}

impl Win32SurfaceCreateInfo<'_> {
    /// Creates a `Win32SurfaceCreateInfo` value from a `Win32WindowHandle`.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::num::NonZeroIsize;
    /// use raw_window_handle::Win32WindowHandle;
    ///
    /// // Construct a sample window handle (fields are illustrative)
    /// let handle = Win32WindowHandle {
    ///     hinstance: NonZeroIsize::new(1),
    ///     hwnd: NonZeroIsize::new(2).unwrap(),
    ///     ..Win32WindowHandle::empty()
    /// };
    ///
    /// let info = Win32SurfaceCreateInfo::from_handle(&handle);
    /// assert_eq!(info.hwnd, handle.hwnd);
    /// ```
    #[inline]
    const fn from_handle(handle: &Win32WindowHandle) -> Self {
        let &Win32WindowHandle {
            hinstance, hwnd, ..
        } = handle;

        Self {
            structure_type: StructureType::Win32SurfaceCreateInfoKhr,
            next: None,
            flags: Win32SurfaceCreateFlags::EMPTY,
            hinstance,
            hwnd,
            _marker: PhantomData,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Win32SurfaceCreateFlags(pub(crate) u32);

impl Win32SurfaceCreateFlags {
    pub const EMPTY: Self = Self(0);
}