use core::{alloc::Layout, ptr::NonNull};

pub struct AllocError;

type AllocResult = Result<NonNull<[u8]>, AllocError>;

pub trait Allocator {
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
