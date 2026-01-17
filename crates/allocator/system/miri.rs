pub mod c_api {
    use std::alloc as rs;

    use core::{alloc::Layout, ptr::NonNull};

    use crate::{AllocError, allocator::AllocResult};

    const LAYOUT_OF_STRUCT_LAYOUT: Layout = Layout::new::<Layout>();

    #[inline]
    pub unsafe fn allocate(data_layout: Layout) -> AllocResult {
        let (block_layout, [layout_offset, data_offset]) = compute_layout(data_layout)?;

        let Some(ptr) = NonNull::new(unsafe { rs::alloc(block_layout) }) else {
            return Err(AllocError);
        };

        // store data layout
        unsafe {
            ptr.byte_add(layout_offset)
                .cast::<Layout>()
                .write(data_layout)
        };

        let data_slice_ptr = NonNull::slice_from_raw_parts(
            unsafe { ptr.byte_add(data_offset).cast() },
            data_layout.size(),
        );

        Ok(data_slice_ptr)
    }

    #[inline]
    pub unsafe fn deallocate(ptr: NonNull<u8>) {
        let data_layout = unsafe {
            // recover data layout from header
            let layout_ptr = {
                ptr.byte_sub(LAYOUT_OF_STRUCT_LAYOUT.size())
                    .cast::<Layout>()
            };

            layout_ptr.read()
        };

        let Ok((block_layout, [_, data_offset])) = compute_layout(data_layout) else {
            unreachable!("There is a memory bug related to layout");
        };

        // deallocate the full block
        let header_ptr = unsafe { ptr.as_ptr().byte_sub(data_offset) };

        unsafe { rs::dealloc(header_ptr, block_layout) };
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
    pub unsafe fn reallocate(old_ptr: NonNull<u8>, data_layout: Layout) -> AllocResult {
        let (new_layout, [new_layout_offset, new_data_offset]) = compute_layout(data_layout)?;

        let (old_layout, [_, data_offset]) = {
            let old_data_layout = unsafe {
                // recover old data layout from header
                let layout_ptr = old_ptr
                    .byte_sub(LAYOUT_OF_STRUCT_LAYOUT.size())
                    .cast::<Layout>();
                layout_ptr.read()
            };

            compute_layout(old_data_layout)?
        };

        let header_ptr = unsafe { old_ptr.byte_sub(data_offset) };

        let ptr = unsafe {
            NonNull::new_unchecked(rs::realloc(
                header_ptr.as_ptr(),
                old_layout,
                new_layout.size(),
            ))
        };

        // store new block layout
        unsafe {
            ptr.byte_add(new_layout_offset)
                .cast::<Layout>()
                .write(data_layout)
        };

        let data_slice_ptr = NonNull::slice_from_raw_parts(
            unsafe { ptr.byte_add(new_data_offset).cast() },
            data_layout.size(),
        );

        Ok(data_slice_ptr)
    }

    #[inline(always)]
    const fn compute_layout(data_layout: Layout) -> Result<(Layout, [usize; 2]), AllocError> {
        let Ok((block_layout, data_offset)) = LAYOUT_OF_STRUCT_LAYOUT.extend(data_layout) else {
            return Err(AllocError);
        };

        let layout_offset = data_offset.saturating_sub(LAYOUT_OF_STRUCT_LAYOUT.size());

        Ok((block_layout, [layout_offset, data_offset]))
    }
}
