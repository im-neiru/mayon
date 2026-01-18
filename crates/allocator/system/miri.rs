use std::alloc as rs;

use core::{
    alloc::Layout,
    ptr::{NonNull, null_mut},
};

const LAYOUT_OF_STRUCT_LAYOUT: Layout = Layout::new::<Layout>();

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn allocate(size: usize, alignment: usize) -> *mut u8 {
    let Ok(data_layout) = Layout::from_size_align(size, alignment) else {
        return null_mut();
    };

    let Some((block_layout, [layout_offset, data_offset])) = compute_layout(data_layout) else {
        return null_mut();
    };

    let Some(ptr) = NonNull::new(rs::alloc(block_layout)) else {
        return null_mut();
    };

    // store data layout

    ptr.byte_add(layout_offset)
        .cast::<Layout>()
        .write(data_layout);

    ptr.byte_add(data_offset).as_ptr()
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn deallocate(ptr: *mut u8) {
    let data_layout = {
        // recover data layout from header
        let layout_ptr = {
            ptr.byte_sub(LAYOUT_OF_STRUCT_LAYOUT.size())
                .cast::<Layout>()
        };

        layout_ptr.read()
    };

    let Some((block_layout, [_, data_offset])) = compute_layout(data_layout) else {
        unreachable!("There is a memory bug related to layout");
    };

    // deallocate the full block
    let header_ptr = ptr.byte_sub(data_offset);

    rs::dealloc(header_ptr, block_layout);
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn reallocate(
    old_ptr: *mut u8,
    new_size: usize,
    alignment: usize,
) -> *mut u8 {
    let Ok(data_layout) = Layout::from_size_align(new_size, alignment) else {
        return null_mut();
    };

    let Some((new_layout, [new_layout_offset, new_data_offset])) = compute_layout(data_layout)
    else {
        return null_mut();
    };

    let Some((old_layout, [_, data_offset])) = ({
        let old_data_layout = {
            // recover old data layout from header
            let layout_ptr = old_ptr
                .byte_sub(LAYOUT_OF_STRUCT_LAYOUT.size())
                .cast::<Layout>();
            layout_ptr.read()
        };

        compute_layout(old_data_layout)
    }) else {
        return null_mut();
    };

    let header_ptr = old_ptr.byte_sub(data_offset);

    let Some(ptr) = NonNull::new(rs::realloc(header_ptr, old_layout, new_layout.size())) else {
        return null_mut();
    };

    // store new block layout

    ptr.byte_add(new_layout_offset)
        .cast::<Layout>()
        .write(data_layout);

    ptr.byte_add(new_data_offset).as_ptr()
}

#[inline(always)]
const fn compute_layout(data_layout: Layout) -> Option<(Layout, [usize; 2])> {
    let Ok((block_layout, data_offset)) = LAYOUT_OF_STRUCT_LAYOUT.extend(data_layout) else {
        return None;
    };

    let layout_offset = data_offset.saturating_sub(LAYOUT_OF_STRUCT_LAYOUT.size());

    Some((block_layout, [layout_offset, data_offset]))
}
