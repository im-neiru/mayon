use core::{
    alloc::{Allocator, Layout},
    ptr::NonNull,
};

impl<A> super::Inner<A>
where
    A: Allocator,
{
    pub(super) unsafe fn store_zeroed<V>(allocator: &A, value: V) -> NonNull<V> {
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
}
