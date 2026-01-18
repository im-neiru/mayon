use core::ffi::c_void;

#[link(name = "msvcrt")]
unsafe extern "C" {
    /// Allocates memory on a specified alignment boundary.
    #[link_name = "_aligned_malloc"]
    unsafe fn aligned_malloc(size: usize, alignment: usize) -> *mut c_void;

    /// Frees a block of memory that was allocated with `_aligned_malloc`.
    #[link_name = "_aligned_free"]
    unsafe fn aligned_free(ptr: *mut c_void);

    /// Changes the size of a memory block that was allocated with `_aligned_malloc`.
    #[link_name = "_aligned_realloc"]
    unsafe fn aligned_realloc(ptr: *mut c_void, new_size: usize, alignment: usize) -> *mut c_void;
}

/// Allocates `size` bytes of memory with the given alignment.
///
/// The returned pointer is aligned to `alignment` bytes and must be freed
/// using [`deallocate`] or reallocated using [`reallocate`].
///
/// On failure, this function returns a null pointer.
///
/// # Safety
///
/// - `alignment` **must** be a power of two and at least `size_of::<usize>()`.
/// - The returned pointer, if non-null, is uninitialized.
/// - The caller must ensure the memory is eventually freed using
///   [`deallocate`] exactly once.
/// - The allocated memory must not be accessed after it is freed.
///
/// # Platform
///
/// This function uses `_aligned_malloc` from the MSVC CRT and is
/// **Windows-only**.
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn allocate(size: usize, alignment: usize) -> *mut u8 {
    aligned_malloc(size, alignment).cast()
}

/// Deallocates memory previously allocated by [`allocate`] or [`reallocate`].
///
/// # Safety
///
/// - `ptr` must have been returned by [`allocate`] or [`reallocate`].
/// - `ptr` must not be null.
/// - `ptr` must not have already been freed.
/// - After calling this function, `ptr` must not be dereferenced or reused.
///
/// # Platform
///
/// This function uses `_aligned_free` from the MSVC CRT and is
/// **Windows-only**.
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn deallocate(ptr: *mut u8) {
    aligned_free(ptr.cast())
}

/// Reallocates a block of memory previously allocated by [`allocate`] or
/// [`reallocate`] to a new size, preserving its alignment.
///
/// On failure, this function returns a null pointer and the original allocation
/// remains valid.
///
/// # Safety
///
/// - `ptr` must have been returned by [`allocate`] or [`reallocate`].
/// - `ptr` must not be null.
/// - `alignment` must be the same alignment originally used to allocate `ptr`.
/// - `alignment` must be a power of two and at least `size_of::<usize>()`.
/// - If reallocation succeeds, the old pointer must no longer be used.
/// - If reallocation fails (null return), the original pointer remains valid
///   and must still be freed by the caller.
///
/// # Platform
///
/// This function uses `_aligned_realloc` from the MSVC CRT and is
/// **Windows-only**.
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn reallocate(ptr: *mut u8, new_size: usize, alignment: usize) -> *mut u8 {
    aligned_realloc(ptr.cast(), new_size, alignment).cast()
}
