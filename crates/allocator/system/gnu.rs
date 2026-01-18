use core::{ffi::c_void, mem, ptr::null_mut};

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

pub unsafe extern "C" fn allocate(size: usize, alignment: usize) -> *mut u8 {
    if alignment < mem::size_of::<usize>() || !alignment.is_power_of_two() {
        return ptr::null_mut();
    }

    let mut ptr: *mut c_void = ptr::null_mut();

    let err = unsafe { posix_memalign(&mut ptr, alignment, size) };

    if err != 0 {
        ptr::null_mut()
    } else {
        ptr.cast()
    }
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn deallocate(ptr: *mut u8) {
    free(ptr.cast())
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn reallocate(
    old_ptr: *mut u8,
    new_size: usize,
    alignment: usize,
) -> *mut u8 {
    let ptr = allocate(new_size, alignment);

    if ptr.is_null() {
        return null_mut();
    }

    let old_size; // todo

    ptr.copy_from_nonoverlapping(old_ptr, old_size.min(new_size));

    ptr
}

struct BlockLayout {
    data_offset: usize,
    size: usize,
    alignment: usize,
}

impl BlockLayout {
    const WORD_SIZE: usize = size_of::<usize>();

    fn new(requested_size: usize, requested_alignment: usize) -> Self {
        let alignment = requested_alignment.max(Self::WORD_SIZE);
        debug_assert!(alignment.is_power_of_two());

        let data_offset = Self::WORD_SIZE.next_multiple_of(alignment);

        let size = data_offset + requested_size;

        Self {
            data_offset,
            size,
            alignment,
        }
    }
}
