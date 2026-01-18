use core::ffi::c_void;

#[link(name = "msvcrt")]
unsafe extern "C" {
    /// Allocates memory on a specified alignment boundary.
    #[link_name = "_aligned_malloc"]
    pub(crate) unsafe fn aligned_malloc(size: usize, alignment: usize) -> *mut c_void;

    /// Frees a block of memory that was allocated with `_aligned_malloc`.
    #[link_name = "_aligned_free"]
    pub(crate) unsafe fn aligned_free(ptr: *mut c_void);

    /// Changes the size of a memory block that was allocated with `_aligned_malloc`.
    #[link_name = "_aligned_realloc"]
    pub(crate) unsafe fn aligned_realloc(
        ptr: *mut c_void,
        new_size: usize,
        alignment: usize,
    ) -> *mut c_void;
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn allocate(size: usize, alignment: usize) -> *mut u8 {
    aligned_malloc(size, alignment).cast()
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn deallocate(ptr: *mut u8) {
    aligned_free(ptr.cast())
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn reallocate(ptr: *mut u8, new_size: usize, alignment: usize) -> *mut u8 {
    aligned_realloc(ptr.cast(), new_size, alignment).cast()
}
