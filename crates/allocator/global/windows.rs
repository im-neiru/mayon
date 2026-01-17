use crate::{Allocator, allocator::AllocResult};

impl Allocator for super::Global {
    unsafe fn allocate(&self, layout: core::alloc::Layout) -> AllocResult {
        todo!()
    }

    unsafe fn allocate_zeroed(&self, layout: core::alloc::Layout) -> AllocResult {
        todo!()
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        todo!()
    }

    unsafe fn grow(
        &self,
        ptr: core::ptr::NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> AllocResult {
        todo!()
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: core::ptr::NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> AllocResult {
        todo!()
    }

    unsafe fn shrink(
        &self,
        ptr: core::ptr::NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> AllocResult {
        todo!()
    }
}
