#[cfg(miri)]
mod miri;

#[cfg(not(miri))]
#[cfg(target_family = "unix")]
mod unix;

#[cfg(not(miri))]
#[cfg(target_family = "windows")]
mod windows;

use core::{alloc::Layout, ptr::NonNull};

use crate::allocator::AllocResult;

pub struct System;

#[cfg(not(miri))]
#[cfg(feature = "for_c_api")]
#[cfg(target_family = "windows")]
pub use windows::c_api;

#[cfg(not(miri))]
#[cfg(feature = "for_c_api")]
#[cfg(target_family = "unix")]
pub use unix::c_api;

#[cfg(miri)]
pub use miri::c_api;

unsafe impl crate::Allocator for super::System {
    #[inline]
    unsafe fn allocate(&self, layout: Layout) -> AllocResult {
        unsafe { c_api::allocate(layout) }
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>) {
        unsafe { c_api::deallocate(ptr) };
    }

    #[inline]
    unsafe fn reallocate(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { c_api::reallocate(ptr, new_layout) }
    }
}
