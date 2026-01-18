use core::{alloc::Layout, ptr::NonNull};

use allocator::raw as sys_raw;
use mayon::allocator::{AllocError, Allocator};

#[repr(C)]
#[derive(Copy, Clone)]

pub struct MynCustomAllocator {
    pub pfn_allocate: unsafe extern "C" fn(size: usize, alignment: usize) -> *mut u8,
    pub pfn_deallocate: unsafe extern "C" fn(ptr: *mut u8),
    pub pfn_reallocate:
        unsafe extern "C" fn(ptr: *mut u8, new_size: usize, alignment: usize) -> *mut u8,
}

unsafe impl Allocator for MynCustomAllocator {
    unsafe fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new((self.pfn_allocate)(layout.size(), layout.align()))
                    .ok_or(AllocError)?,
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
                NonNull::new((self.pfn_reallocate)(
                    ptr.as_mut(),
                    new_layout.size(),
                    new_layout.align(),
                ))
                .ok_or(AllocError)?,
                new_layout.size(),
            ))
        }
    }
}

impl MynCustomAllocator {
    pub(crate) const DEFAULT: Self = Self {
        pfn_allocate: sys_raw::allocate,
        pfn_deallocate: sys_raw::deallocate,
        pfn_reallocate: sys_raw::reallocate,
    };
}
