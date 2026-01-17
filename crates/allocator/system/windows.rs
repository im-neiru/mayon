pub mod c_api {
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

    /// Allocates uninitialized memory from the process heap.
    ///
    /// # Safety
    ///
    /// - The returned memory is **uninitialized** and must be fully initialized
    ///   before being read.
    /// - The allocation must eventually be freed using [`deallocate`].
    /// - Alignment requirements are not guaranteed by the Windows heap.
    #[inline]
    pub unsafe fn allocate(layout: Layout) -> AllocResult {
        let ptr = unsafe { heap_alloc(get_process_heap(), 0, layout.size()) as *mut u8 };

        let Some(ptr) = NonNull::new(ptr) else {
            return Err(AllocError);
        };

        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    /// Frees a memory block previously allocated from the process heap.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been returned by [`allocate`],
    ///   or a successful reallocation.
    /// - `ptr` must not be freed more than once.
    /// - No references to the allocation may be used after this call.
    #[inline]
    pub unsafe fn deallocate(ptr: NonNull<u8>) {
        unsafe {
            heap_free(get_process_heap(), 0, ptr.as_ptr() as *mut _);
        }
    }

    /// Reallocates an existing allocation to a new layout.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid allocation from this module.
    /// - Any existing references to the allocation must not be used after
    ///   this call.
    /// - The returned memory must eventually be freed using [`deallocate`].
    #[inline]
    pub unsafe fn reallocate(ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        let ptr = unsafe {
            heap_realloc(
                get_process_heap(),
                0,
                ptr.as_ptr().cast(),
                new_layout.size(),
            ) as *mut u8
        };

        let Some(ptr) = NonNull::new(ptr) else {
            return Err(AllocError);
        };

        Ok(NonNull::slice_from_raw_parts(ptr, new_layout.size()))
    }
}
