use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

use std::alloc as default_alloc;

#[repr(C)]
#[derive(Copy, Clone)]

pub struct MynCustomAllocator {
    pub pfn_allocate: unsafe extern "C" fn(layout: MynMemLayout) -> *mut u8,
    pub pfn_allocate_zeroed: unsafe extern "C" fn(layout: MynMemLayout) -> *mut u8,
    pub pfn_deallocate: unsafe extern "C" fn(ptr: *const u8, layout: MynMemLayout),
    pub pfn_grow: unsafe extern "C" fn(
        ptr: *mut u8,
        old_layout: MynMemLayout,
        new_layout: MynMemLayout,
    ) -> *mut u8,
    pub pfn_grow_zeroed: unsafe extern "C" fn(
        ptr: *mut u8,
        old_layout: MynMemLayout,
        new_layout: MynMemLayout,
    ) -> *mut u8,
    pub pfn_shrink: unsafe extern "C" fn(
        ptr: *mut u8,
        old_layout: MynMemLayout,
        new_layout: MynMemLayout,
    ) -> *mut u8,
}

unsafe impl Allocator for MynCustomAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_allocate)(layout.into())).ok_or(AllocError)?,
                layout.size(),
            ))
        }
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_allocate_zeroed)(layout.into())).ok_or(AllocError)?,
                layout.size(),
            ))
        }
    }

    unsafe fn deallocate(&self, mut ptr: NonNull<u8>, layout: Layout) {
        unsafe { (self.pfn_deallocate)(ptr.as_mut(), layout.into()) }
    }

    unsafe fn grow(
        &self,
        mut ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_grow)(
                    ptr.as_mut(),
                    old_layout.into(),
                    new_layout.into(),
                ))
                .ok_or(AllocError)?,
                new_layout.size(),
            ))
        }
    }

    unsafe fn grow_zeroed(
        &self,
        mut ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_grow_zeroed)(
                    ptr.as_mut(),
                    old_layout.into(),
                    new_layout.into(),
                ))
                .ok_or(AllocError)?,
                new_layout.size(),
            ))
        }
    }

    unsafe fn shrink(
        &self,
        mut ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_shrink)(
                    ptr.as_mut(),
                    old_layout.into(),
                    new_layout.into(),
                ))
                .ok_or(AllocError)?,
                new_layout.size(),
            ))
        }
    }
}

impl MynCustomAllocator {
    pub(crate) const DEFAULT: Self = Self {
        pfn_allocate: Self::allocate,
        pfn_allocate_zeroed: Self::allocate_zeroed,
        pfn_deallocate: Self::deallocate,
        pfn_grow: Self::realloc,
        pfn_grow_zeroed: Self::grow_zeroed,
        pfn_shrink: Self::realloc,
    };

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn allocate(layout: MynMemLayout) -> *mut u8 {
        default_alloc::alloc(layout.into())
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn allocate_zeroed(layout: MynMemLayout) -> *mut u8 {
        default_alloc::alloc_zeroed(layout.into())
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn deallocate(ptr: *const u8, layout: MynMemLayout) {
        default_alloc::dealloc(ptr as *mut u8, layout.into());
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn realloc(
        ptr: *mut u8,
        old_layout: MynMemLayout,
        new_layout: MynMemLayout,
    ) -> *mut u8 {
        default_alloc::realloc(ptr, old_layout.into(), new_layout.size)
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn grow_zeroed(
        ptr: *mut u8,
        old_layout: MynMemLayout,
        new_layout: MynMemLayout,
    ) -> *mut u8 {
        let ptr = default_alloc::realloc(ptr, old_layout.into(), new_layout.size);

        if !ptr.is_null() && (new_layout.size > old_layout.size) {
            ptr.add(old_layout.size)
                .write_bytes(0, new_layout.size - old_layout.size);
        }

        ptr
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MynMemLayout {
    size: usize,
    alignment: usize,
}

impl From<Layout> for MynMemLayout {
    #[inline]
    fn from(layout: Layout) -> Self {
        Self {
            size: layout.size(),
            alignment: layout.align(),
        }
    }
}

impl From<MynMemLayout> for Layout {
    #[inline]
    fn from(layout: MynMemLayout) -> Self {
        unsafe { Self::from_size_align_unchecked(layout.size, layout.alignment) }
    }
}
