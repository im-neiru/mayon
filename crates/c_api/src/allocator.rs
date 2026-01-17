use core::{
    alloc::Layout,
    ptr::{NonNull, null_mut},
};

use allocator::c_api;
use mayon::allocator::{AllocError, Allocator};

#[repr(C)]
#[derive(Copy, Clone)]

pub struct MynCustomAllocator {
    pub pfn_allocate: unsafe extern "C" fn(layout: MynMemLayout) -> *mut u8,
    pub pfn_allocate_zeroed: unsafe extern "C" fn(layout: MynMemLayout) -> *mut u8,
    pub pfn_deallocate: unsafe extern "C" fn(ptr: *const u8),
    pub pfn_reallocate: unsafe extern "C" fn(ptr: *mut u8, new_layout: MynMemLayout) -> *mut u8,
    pub pfn_reallocate_zeroed:
        unsafe extern "C" fn(ptr: *mut u8, new_layout: MynMemLayout) -> *mut u8,
}

unsafe impl Allocator for MynCustomAllocator {
    unsafe fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_allocate)(layout.into())).ok_or(AllocError)?,
                layout.size(),
            ))
        }
    }

    unsafe fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_allocate_zeroed)(layout.into())).ok_or(AllocError)?,
                layout.size(),
            ))
        }
    }

    unsafe fn deallocate(&self, mut ptr: NonNull<u8>) {
        unsafe { (self.pfn_deallocate)(ptr.as_mut()) }
    }

    unsafe fn reallocate(
        &self,
        mut ptr: NonNull<u8>,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_reallocate)(ptr.as_mut(), new_layout.into()))
                    .ok_or(AllocError)?,
                new_layout.size(),
            ))
        }
    }

    unsafe fn reallocate_zeroed(
        &self,
        mut ptr: NonNull<u8>,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_reallocate_zeroed)(
                    ptr.as_mut(),
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
        pfn_reallocate: Self::reallocate,
        pfn_reallocate_zeroed: Self::reallocate_zeroed,
    };

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn allocate(layout: MynMemLayout) -> *mut u8 {
        if let Ok(ptr) = c_api::allocate(layout.into()) {
            ptr.as_ptr().cast()
        } else {
            null_mut()
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn allocate_zeroed(layout: MynMemLayout) -> *mut u8 {
        if let Ok(ptr) = c_api::allocate_zeroed(layout.into()) {
            ptr.as_ptr().cast()
        } else {
            null_mut()
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn deallocate(ptr: *const u8) {
        c_api::deallocate(NonNull::new_unchecked(ptr.cast_mut()));
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn reallocate(ptr: *mut u8, new_layout: MynMemLayout) -> *mut u8 {
        if let Ok(ptr) = c_api::reallocate(NonNull::new_unchecked(ptr), new_layout.into()) {
            ptr.as_ptr().cast()
        } else {
            null_mut()
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn reallocate_zeroed(ptr: *mut u8, new_layout: MynMemLayout) -> *mut u8 {
        if let Ok(ptr) = c_api::reallocate_zeroed(NonNull::new_unchecked(ptr), new_layout.into()) {
            ptr.as_ptr().cast()
        } else {
            null_mut()
        }
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
