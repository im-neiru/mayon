use core::{
    alloc::{AllocError, Allocator, Layout},
    cmp::Ordering::*,
    ptr::NonNull,
};

pub trait AllocatorUtils {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn allocate_with_stored_layout(
        &self,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError>;

    #[allow(clippy::missing_safety_doc)]
    unsafe fn reallocate_with_stored_layout(
        &self,
        old_ptr: NonNull<u8>,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError>;

    #[allow(clippy::missing_safety_doc)]
    unsafe fn deallocate_with_stored_layout(&self, ptr: NonNull<u8>);
}

impl<A> AllocatorUtils for A
where
    A: Allocator,
{
    #[inline]
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn allocate_with_stored_layout(
        &self,
        data_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let (block_layout, [layout_offset, data_offset]) = compute_layout(data_layout)?;

        let ptr = self.allocate(block_layout)?;

        // store data layout
        ptr.byte_add(layout_offset)
            .cast::<Layout>()
            .write(data_layout);

        let data_slice_ptr =
            NonNull::slice_from_raw_parts(ptr.byte_add(data_offset).cast(), data_layout.size());

        Ok(data_slice_ptr)
    }

    #[inline]
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn reallocate_with_stored_layout(
        &self,
        old_ptr: NonNull<u8>,
        data_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let (new_layout, [new_layout_offset, new_data_offset]) = compute_layout(data_layout)?;

        let (old_layout, [_, data_offset]) = {
            let old_data_layout = {
                // recover old data layout from header
                let layout_ptr = old_ptr
                    .byte_sub(LAYOUT_OF_STRUCT_LAYOUT.size())
                    .cast::<Layout>();
                layout_ptr.read()
            };

            compute_layout(old_data_layout)?
        };

        let header_ptr = old_ptr.byte_sub(data_offset);

        let ptr = match new_layout.size().cmp(&old_layout.size()) {
            Greater => self.grow(header_ptr, old_layout, new_layout)?,
            Less => self.shrink(header_ptr, old_layout, new_layout)?,
            Equal => {
                return Ok(NonNull::slice_from_raw_parts(old_ptr, old_layout.size()));
            }
        };

        // store new block layout
        ptr.byte_add(new_layout_offset)
            .cast::<Layout>()
            .write(new_layout);

        let data_slice_ptr =
            NonNull::slice_from_raw_parts(ptr.byte_add(new_data_offset).cast(), data_layout.size());

        Ok(data_slice_ptr)
    }

    #[inline]
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn deallocate_with_stored_layout(&self, ptr: NonNull<u8>) {
        let data_layout = {
            // recover data layout from header
            let layout_ptr = ptr
                .byte_sub(LAYOUT_OF_STRUCT_LAYOUT.size())
                .cast::<Layout>();
            layout_ptr.read()
        };

        let Ok((block_layout, [_, data_offset])) = compute_layout(data_layout) else {
            unreachable!("There is a memory bug related to layout");
        };

        // deallocate the full block
        let header_ptr = ptr.as_ptr().byte_sub(data_offset);
        self.deallocate(NonNull::new_unchecked(header_ptr), block_layout);
    }
}

const LAYOUT_OF_STRUCT_LAYOUT: Layout = Layout::new::<Layout>();

#[inline(always)]
const fn compute_layout(data_layout: Layout) -> Result<(Layout, [usize; 2]), AllocError> {
    let Ok((block_layout, data_offset)) = LAYOUT_OF_STRUCT_LAYOUT.extend(data_layout) else {
        return Err(AllocError);
    };

    let layout_offset = data_offset.saturating_sub(LAYOUT_OF_STRUCT_LAYOUT.size());

    Ok((block_layout, [layout_offset, data_offset]))
}
