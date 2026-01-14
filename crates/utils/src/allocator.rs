use core::{
    alloc::{AllocError, Allocator, Layout},
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
            layout_dest.write(data_layout);
        }

        Ok(NonNull::slice_from_raw_parts(
            ptr.byte_add(LAYOUT_OF_STRUCT_LAYOUT.size()).cast(),
            data_layout.size(),
        ))
    }

    unsafe fn reallocate_with_stored_layout(
        &self,
        old_ptr: NonNull<u8>,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }

    unsafe fn deallocate_with_stored_layout(&self, ptr: NonNull<u8>) {
        todo!()
    }
}

const LAYOUT_OF_STRUCT_LAYOUT: Layout = Layout::new::<Layout>();
