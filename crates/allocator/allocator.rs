use core::{alloc::Layout, ptr::NonNull};

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Allocation Error")]
pub struct AllocError;

pub(crate) type AllocResult = Result<NonNull<[u8]>, AllocError>;

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    unsafe fn allocate(&self, layout: Layout) -> AllocResult;

    unsafe fn allocate_zeroed(&self, layout: Layout) -> AllocResult;

    unsafe fn deallocate(&self, ptr: NonNull<u8>);

    unsafe fn reallocate(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult;

    unsafe fn reallocate_zeroed(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult;

    #[inline]
    unsafe fn grow(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { self.reallocate(ptr, new_layout) }
    }

    #[inline]
    unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { self.reallocate_zeroed(ptr, new_layout) }
    }

    #[inline]
    unsafe fn shrink(&self, ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { self.reallocate(ptr, new_layout) }
    }
}
