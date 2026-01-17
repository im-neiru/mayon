use core::{alloc::Layout, ptr::NonNull};

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Allocation Error")]
pub struct AllocError;

pub(crate) type AllocResult = Result<NonNull<[u8]>, AllocError>;

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    unsafe fn allocate(&self, layout: Layout) -> AllocResult;

    unsafe fn allocate_zeroed(&self, layout: Layout) -> AllocResult;

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);

    unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> AllocResult;

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> AllocResult;

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> AllocResult;
}
