use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

pub trait AllocatorUtils {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn allocate_with_stored_layout(
        &self,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError>;

    #[allow(clippy::missing_safety_doc)]
    unsafe fn reallocate_with_stored_layout(
        &self,
        old_ptr: NonNull<u8>,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError>;

    #[allow(clippy::missing_safety_doc)]
    unsafe fn deallocate_with_stored_layout(&self, ptr: NonNull<u8>);
}

impl<A> AllocatorUtils for A
where
    A: Allocator,
{
    unsafe fn allocate_with_stored_layout(
        &self,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }

    unsafe fn reallocate_with_stored_layout(
        &self,
        old_ptr: NonNull<u8>,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }

    unsafe fn deallocate_with_stored_layout(&self, ptr: NonNull<u8>) {
        todo!()
    }
}
