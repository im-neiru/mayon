use core::{ffi::c_void, ptr::null_mut};

unsafe extern "C" {
    /// C11 aligned allocation.
    ///
    /// Requires:
    /// - `alignment` is a power of two
    /// - `size` is a multiple of `alignment`
    ///
    /// Returns null on failure.
    pub(crate) unsafe fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void;

    /// Frees memory allocated by `aligned_alloc`, `posix_memalign`, or `malloc`.
    pub(crate) unsafe fn free(ptr: *mut c_void);
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn allocate(size: usize, alignment: usize) -> *mut u8 {
    let layout = BlockLayout::new(size, alignment);

    let start_ptr = unsafe { aligned_alloc(layout.alignment, layout.size) as *mut u8 };

    if start_ptr.is_null() {
        return null_mut();
    }

    let ptr = start_ptr.byte_add(layout.data_offset);

    ptr.byte_sub(HEADER_SIZE)
        .cast::<Header>()
        .write(Header { size, start_ptr });

    ptr
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn deallocate(ptr: *mut u8) {
    let Some(header) = ptr.byte_sub(HEADER_SIZE).cast::<Header>().as_ref() else {
        return;
    };

    free(header.start_ptr.cast())
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn reallocate(
    old_ptr: *mut u8,
    new_size: usize,
    alignment: usize,
) -> *mut u8 {
    let Some(&Header {
        size: old_size,
        start_ptr: old_start_ptr,
    }) = old_ptr.byte_sub(HEADER_SIZE).cast::<Header>().as_ref()
    else {
        return null_mut();
    };

    let ptr = allocate(new_size, alignment);

    if ptr.is_null() {
        return null_mut();
    }

    ptr.copy_from_nonoverlapping(old_ptr, old_size.min(new_size));

    free(old_start_ptr.cast());

    ptr
}

const HEADER_SIZE: usize = size_of::<Header>();

#[cfg_attr(target_pointer_width = "64", repr(C, align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(C, align(4)))]
#[derive(Clone, Copy, Debug)]
struct Header {
    size: usize,
    start_ptr: *mut u8,
}

struct BlockLayout {
    data_offset: usize,
    size: usize,
    alignment: usize,
}

impl BlockLayout {
    #[inline(always)]
    fn new(requested_size: usize, requested_alignment: usize) -> Self {
        let alignment = requested_alignment.max(HEADER_SIZE);
        debug_assert!(alignment.is_power_of_two());

        let data_offset = HEADER_SIZE.next_multiple_of(alignment);
        let size = (data_offset + requested_size).next_multiple_of(alignment);

        Self {
            data_offset,
            size,
            alignment,
        }
    }
}
