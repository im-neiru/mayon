use core::ffi::c_void;

unsafe extern "C" {
    /// C11 aligned allocation.
    ///
    /// Requires:
    /// - `alignment` is a power of two
    /// - `size` is a multiple of `alignment`
    ///
    /// Returns null on failure.
    pub(crate) unsafe fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void;

    /// POSIX-aligned allocation.
    ///
    /// Returns:
    /// - 0 on success
    /// - error code (EINVAL / ENOMEM) on failure
    pub(crate) unsafe fn posix_memalign(
        memptr: *mut *mut c_void,
        alignment: usize,
        size: usize,
    ) -> i32;

    /// Frees memory allocated by `aligned_alloc`, `posix_memalign`, or `malloc`.
    pub(crate) unsafe fn free(ptr: *mut c_void);
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn allocate(size: usize, alignment: usize) -> *mut u8 {
    if alignment < core::mem::size_of::<usize>() || !alignment.is_power_of_two() {
        return core::ptr::null_mut();
    }

    let mut ptr: *mut c_void = core::ptr::null_mut();
    let err = posix_memalign(&mut ptr, alignment, size);

    if err != 0 {
        core::ptr::null_mut()
    } else {
        ptr.cast()
    }
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn deallocate(ptr: *mut u8) {
    free(ptr.cast())
}
