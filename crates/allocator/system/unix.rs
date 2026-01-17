pub mod c_api {
    use core::{alloc::Layout, ffi::c_void, ptr::NonNull};

    use crate::{AllocError, allocator::AllocResult};

    unsafe extern "C" {
        /// Allocates `size` bytes of uninitialized memory.
        ///
        /// The returned pointer must be freed with `free`.
        unsafe fn malloc(size: usize) -> *mut c_void;

        /// Reallocates a memory block previously allocated by `malloc`,
        /// or `realloc`.
        ///
        /// The returned pointer may differ from the original pointer.
        unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;

        /// Frees a memory block previously allocated by `malloc`,
        /// or `realloc`.
        unsafe fn free(ptr: *mut c_void);
    }

    #[inline]
    unsafe fn inner_allocate(layout: Layout) -> AllocResult {
        let ptr = unsafe { malloc(layout.size()) as *mut u8 };

        let Some(ptr) = NonNull::new(ptr) else {
            return Err(AllocError);
        };

        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    #[inline]
    unsafe fn inner_reallocate(ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        let ptr = unsafe { realloc(ptr.as_ptr().cast(), new_layout.size()) as *mut u8 };

        let Some(ptr) = NonNull::new(ptr) else {
            return Err(AllocError);
        };

        Ok(NonNull::slice_from_raw_parts(ptr, new_layout.size()))
    }

    /// Allocates uninitialized memory.
    ///
    /// # Safety
    ///
    /// - The returned memory is **uninitialized**.
    /// - The allocation must eventually be freed using [`deallocate`].
    /// - Alignment guarantees are platform-dependent.
    #[inline]
    pub unsafe fn allocate(layout: Layout) -> AllocResult {
        unsafe { inner_allocate(layout) }
    }

    /// Frees a memory block previously allocated by this module.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been returned by [`allocate`], [`allocate_zeroed`],
    ///   or a successful reallocation.
    /// - `ptr` must not be freed more than once.
    /// - No references to the allocation may be used after this call.
    #[inline]
    pub unsafe fn deallocate(ptr: NonNull<u8>) {
        unsafe {
            free(ptr.as_ptr().cast());
        }
    }

    /// Reallocates an existing allocation to a new layout.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid allocation from this module.
    /// - Any existing references to the allocation must not be used after
    ///   this call.
    /// - The returned allocation must eventually be freed using [`deallocate`].
    #[inline]
    pub unsafe fn reallocate(ptr: NonNull<u8>, new_layout: Layout) -> AllocResult {
        unsafe { inner_reallocate(ptr, new_layout) }
    }
}
