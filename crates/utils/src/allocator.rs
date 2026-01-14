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
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn allocate_with_stored_layout(
        &self,
        data_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let Ok(block_layout) = LAYOUT_OF_STRUCT_LAYOUT.extend_packed(data_layout) else {
            return Err(AllocError);
        };

        let ptr = self.allocate(block_layout)?;

        {
            // store layout
            let layout_dest = ptr.as_ptr().cast::<Layout>();
            layout_dest.write(block_layout);
        }

        Ok(NonNull::slice_from_raw_parts(
            ptr.byte_add(LAYOUT_OF_STRUCT_LAYOUT.size()).cast(),
            data_layout.size(),
        ))
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn reallocate_with_stored_layout(
        &self,
        old_ptr: NonNull<u8>,
        data_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let Ok(new_layout) = LAYOUT_OF_STRUCT_LAYOUT.extend_packed(data_layout) else {
            return Err(AllocError);
        };

        let old_layout = {
            let layout_ptr = old_ptr
                .as_ptr()
                .byte_sub(LAYOUT_OF_STRUCT_LAYOUT.size())
                .cast::<Layout>();

            layout_ptr.read()
        };

        let ptr = match new_layout.size().cmp(&old_layout.size()) {
            Greater => self.grow(old_ptr, old_layout, new_layout)?,
            Less => self.shrink(old_ptr, old_layout, new_layout)?,
            Equal => {
                return Ok(NonNull::slice_from_raw_parts(old_ptr, old_layout.size()));
            }
        };

        {
            // store layout
            let layout_dest = ptr.as_ptr().cast::<Layout>();
            layout_dest.write(new_layout);
        }

        Ok(NonNull::slice_from_raw_parts(
            ptr.byte_add(LAYOUT_OF_STRUCT_LAYOUT.size()).cast(),
            data_layout.size(),
        ))
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn deallocate_with_stored_layout(&self, ptr: NonNull<u8>) {
        let block_layout = {
            let layout_ptr = ptr
                .as_ptr()
                .byte_sub(LAYOUT_OF_STRUCT_LAYOUT.size())
                .cast::<Layout>();

            layout_ptr.read()
        };

        self.deallocate(ptr, block_layout);
    }
}

const LAYOUT_OF_STRUCT_LAYOUT: Layout = Layout::new::<Layout>();
