use core::{alloc::Layout, ffi::c_void, ptr::NonNull};

use crate::{AllocError, allocator::AllocResult};

#[link(name = "kernel32")]
unsafe extern "system" {

    /// Retrieves a handle to the default heap of the calling process.
    /// This handle can then be used in subsequent calls to the heap functions.
    #[link_name = "GetProcessHeap"]
    unsafe fn get_process_heap() -> *mut c_void;

    /// Allocates a block of memory from a heap. The allocated memory is not movable.
    #[link_name = "HeapAlloc"]
    unsafe fn heap_alloc(heap: *mut c_void, flags: u32, size: usize) -> *mut c_void;

    /// Frees a memory block allocated from a heap by the HeapAlloc or HeapReAlloc function.
    #[link_name = "HeapFree"]
    unsafe fn heap_free(heap: *mut c_void, flags: u32, ptr: *mut c_void) -> i32;

    /// Reallocates a block of memory from a heap.
    /// This function enables you to resize a memory block and change other memory block properties.
    /// The allocated memory is not movable.
    #[link_name = "HeapReAlloc"]
    unsafe fn heap_realloc(
        heap: *mut c_void,
        flags: u32,
        ptr: *mut c_void,
        new_size: usize,
    ) -> *mut c_void;
}

const HEAP_ZERO_MEMORY: u32 = 0x00000008;

#[inline]
unsafe fn allocate(layout: Layout, flags: u32) -> AllocResult {
    let ptr = unsafe { heap_alloc(get_process_heap(), flags, layout.size()) as *mut u8 };

    let Some(ptr) = NonNull::new(ptr) else {
        return Err(AllocError);
    };

    Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
}

#[inline]
unsafe fn rellocate(ptr: NonNull<u8>, new_layout: Layout, flags: u32) -> AllocResult {
    let ptr = unsafe {
        heap_realloc(
            get_process_heap(),
            flags,
            ptr.as_ptr().cast(),
            new_layout.size(),
        ) as *mut u8
    };

    let Some(ptr) = NonNull::new(ptr) else {
        return Err(AllocError);
    };

    Ok(NonNull::slice_from_raw_parts(ptr, new_layout.size()))
}

unsafe impl crate::Allocator for super::Global {
    #[inline]
    unsafe fn allocate(&self, layout: Layout) -> AllocResult {
        unsafe { allocate(layout, 0) }
    }

    #[inline]
    unsafe fn allocate_zeroed(&self, layout: Layout) -> AllocResult {
        unsafe { allocate(layout, HEAP_ZERO_MEMORY) }
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
        unsafe { heap_free(get_process_heap(), 0, ptr.as_ptr() as *mut _) };
    }

    #[inline]
    unsafe fn grow(&self, ptr: NonNull<u8>, _: Layout, new_layout: Layout) -> AllocResult {
        unsafe { rellocate(ptr, new_layout, 0) }
    }

    #[inline]
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        _old_layout: Layout,
        new_layout: Layout,
    ) -> AllocResult {
        unsafe { rellocate(ptr, new_layout, HEAP_ZERO_MEMORY) }
    }

    #[inline]
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        _old_layout: Layout,
        new_layout: Layout,
    ) -> AllocResult {
        unsafe { rellocate(ptr, new_layout, 0) }
    }
}
