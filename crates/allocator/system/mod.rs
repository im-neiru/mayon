#[cfg(miri)]
#[path = "windows/mod.rs"]
pub mod raw;

#[cfg(not(miri))]
#[cfg(target_family = "unix")]
#[path = "windows/mod.rs"]
pub mod raw;

#[cfg(not(miri))]
#[cfg(target_family = "windows")]
#[path = "windows/mod.rs"]
pub mod raw;

use core::{alloc::Layout, ptr::NonNull};

use crate::{AllocError, allocator::AllocResult};

pub struct System;

unsafe impl crate::Allocator for crate::System {
    #[inline]
    unsafe fn allocate(&self, layout: Layout) -> AllocResult {
        let Some(ptr) = NonNull::new(unsafe { raw::allocate(layout.size(), layout.align()) })
        else {
            return Err(AllocError);
        };

        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>) {
        unsafe { raw::deallocate(ptr.as_ptr()) };
    }

    #[inline]
    unsafe fn reallocate(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        let Some(ptr) = NonNull::new(unsafe {
            raw::reallocate(ptr.as_ptr(), new_layout.size(), new_layout.align())
        }) else {
            return Err(AllocError);
        };

        Ok(NonNull::slice_from_raw_parts(ptr, new_layout.size()))
    }
}
