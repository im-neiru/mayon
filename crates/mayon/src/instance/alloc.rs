use core::{
    alloc::{Allocator, Layout},
    ptr::NonNull,
};

pub(super) unsafe fn store_zeroed<A, V>(allocator: &A, value: V) -> NonNull<V>
where
    A: Allocator,
{
    let layout = Layout::new::<V>();

    let Ok(ptr) = allocator
        .allocate_zeroed(layout)
        .map(NonNull::<_>::cast::<V>)
    else {
        std::alloc::handle_alloc_error(layout);
    };

    unsafe {
        ptr.write(value);
    }

    ptr
}
