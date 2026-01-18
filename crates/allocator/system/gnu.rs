use core::{ffi::c_void, ptr::null_mut};

unsafe extern "C" {
    /// C11 aligned allocation.
    ///
    /// Requires:
    /// - `alignment` is a power of two
    /// - `size` is a multiple of `alignment`
    ///
    /// Returns null on failure.
    unsafe fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void;

    /// Frees memory allocated by `aligned_alloc`, or `malloc`.
    unsafe fn free(ptr: *mut c_void);
}

/// Allocates a block of memory with a specific size and alignment.
///
/// This function allocates a larger block than requested to store internal metadata
/// immediately preceding the returned pointer.
///
/// # Safety
///
/// - `alignment` must be a power of two.
/// - The caller is responsible for ensuring the memory is eventually deallocated
///   using the [`deallocate`] function to avoid memory leaks.
///
/// # Returns
///
/// - On success: A pointer to the start of the requested data block, aligned to `alignment`.
/// - On failure: A null pointer (e.g., if `alignment` is invalid or `aligned_alloc` fails).
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn allocate(size: usize, alignment: usize) -> *mut u8 {
    let Some(layout) = BlockLayout::new(size, alignment) else {
        return null_mut();
    };

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

/// Deallocates a memory block previously allocated by [`allocate`].
///
/// This function retrieves the original allocation metadata from the header
/// stored behind the pointer and frees the entire block.
///
/// # Safety
///
/// - `ptr` must have been returned by a previous call to [`allocate`] or [`reallocate`].
/// - `ptr` must not have been previously deallocated (no double-free).
/// - The memory must not be accessed after this call.
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn deallocate(ptr: *mut u8) {
    let Some(header) = ptr.byte_sub(HEADER_SIZE).cast::<Header>().as_ref() else {
        return;
    };

    free(header.start_ptr.cast())
}

/// Changes the size of an existing memory allocation.
///
/// This function allocates a new block of `new_size`, copies the minimum of
/// `old_size` and `new_size` from the old block to the new one, and then
/// deallocates the old block.
///
/// # Safety
///
/// - `old_ptr` must be a valid pointer previously returned by [`allocate`] or [`reallocate`].
/// - `alignment` must be a power of two.
/// - The same safety rules as [`allocate`] and [`deallocate`] apply here regarding
///   memory access and future cleanup.
///
/// # Returns
///
/// - On success: A pointer to the new memory block.
/// - On failure: A null pointer. Note that if reallocation fails, the **original** ///   memory block is not freed and remains valid.
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
    fn new(requested_size: usize, requested_alignment: usize) -> Option<Self> {
        let alignment = requested_alignment.max(HEADER_SIZE);
        debug_assert!(alignment.is_power_of_two());

        let data_offset = HEADER_SIZE.next_multiple_of(alignment);

        let size = data_offset
            .checked_add(requested_size)?
            .next_multiple_of(alignment);

        Some(Self {
            data_offset,
            size,
            alignment,
        })
    }
}
