use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::{NonNull, slice_from_raw_parts_mut},
};

#[repr(C)]
#[derive(Copy, Clone)]

pub struct MynCustomAllocator {
    pub pfn_allocate: unsafe extern "C" fn(layout: MynMemLayout) -> *mut u8,
    pub pfn_allocate_zeroed: unsafe extern "C" fn(layout: MynMemLayout) -> *mut u8,
    pub pfn_deallocate: unsafe extern "C" fn(ptr: *mut u8, layout: MynMemLayout),
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
