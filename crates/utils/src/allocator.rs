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
            .write(data_layout);

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

#[cfg(test)]
mod tests {
    use super::*;

    use std::alloc::Global;

    #[test]
    fn allocate_and_deallocate_simple() {
        unsafe {
            let alloc = Global;

            let layout = Layout::from_size_align(32, 8).unwrap();
            let mut ptr = alloc.allocate_with_stored_layout(layout).unwrap();

            for (index, dest) in ptr.as_mut().iter_mut().enumerate() {
                *dest = index as u8;
            }

            alloc.deallocate_with_stored_layout(ptr.cast());
        }
    }

    #[test]
    fn allocation_alignment_is_respected() {
        unsafe {
            let alloc = Global;

            let alignments = [1, 2, 4, 8, 16, 32, 64];

            for &align in &alignments {
                let layout = Layout::from_size_align(24, align).unwrap();
                let ptr = alloc.allocate_with_stored_layout(layout).unwrap();

                let addr = ptr.addr().get();
                assert_eq!(addr % align, 0, "alignment {} not respected", align);

                alloc.deallocate_with_stored_layout(ptr.cast());
            }
        }
    }

    #[test]
    fn reallocate_grow_preserves_data() {
        unsafe {
            let alloc = Global;

            let layout1 = Layout::from_size_align(16, 8).unwrap();
            let layout2 = Layout::from_size_align(64, 8).unwrap();

            let mut ptr = alloc.allocate_with_stored_layout(layout1).unwrap();

            for (index, dest) in ptr.as_mut().iter_mut().enumerate() {
                *dest = index as u8;
            }

            let mut ptr = alloc
                .reallocate_with_stored_layout(ptr.cast(), layout2)
                .unwrap();

            for (index, dest) in ptr.as_mut().iter_mut().enumerate() {
                *dest = index as u8;
            }

            alloc.deallocate_with_stored_layout(ptr.cast());
        }
    }

    #[test]
    fn reallocate_shrink_preserves_data() {
        unsafe {
            let alloc = Global;

            let layout1 = Layout::from_size_align(64, 16).unwrap();
            let layout2 = Layout::from_size_align(16, 16).unwrap();

            let mut ptr = alloc.allocate_with_stored_layout(layout1).unwrap();

            for (index, dest) in ptr.as_mut().iter_mut().enumerate() {
                *dest = index as u8;
            }

            let mut ptr = alloc
                .reallocate_with_stored_layout(ptr.cast(), layout2)
                .unwrap();

            for (index, dest) in ptr.as_mut().iter_mut().enumerate() {
                *dest = index as u8;
            }

            alloc.deallocate_with_stored_layout(ptr.cast());
        }
    }

    #[test]
    fn multiple_allocations_do_not_overlap() {
        unsafe {
            let alloc = Global;

            let a = alloc
                .allocate_with_stored_layout(Layout::from_size_align(32, 8).unwrap())
                .unwrap();
            let b = alloc
                .allocate_with_stored_layout(Layout::from_size_align(32, 8).unwrap())
                .unwrap();

            let a_addr = a.addr().get();
            let b_addr = b.addr().get();

            assert_ne!(a_addr, b_addr);

            alloc.deallocate_with_stored_layout(a.cast());
            alloc.deallocate_with_stored_layout(b.cast());
        }
    }
}
