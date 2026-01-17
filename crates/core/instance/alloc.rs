use core::{
    alloc::Layout,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};
use std::alloc::handle_alloc_error;

use allocator::Allocator;

use crate::Backend;

/// Allocates memory for `value` using `allocator` and stores the value in the allocated slot.
///
/// # Safety
///
/// - `allocator` must be a valid allocator able to allocate a block with `Layout::new::<V>()`.
/// - The returned pointer must be deallocated using the same allocator and layout to avoid undefined behavior.
/// - The caller is responsible for ensuring the allocated value is dropped exactly once.
///
/// # Returns
///
/// A `NonNull<V>` pointing to the stored value.
#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
pub(super) unsafe fn allocate<A, V>(allocator: &A, value: V) -> NonNull<V>
where
    A: Allocator,
{
    let layout = Layout::new::<V>();

    let Ok(ptr) = allocator.allocate(layout).map(NonNull::<_>::cast::<V>) else {
        handle_alloc_error(layout);
    };

    unsafe {
        ptr.write(value);
    }

    ptr
}

#[inline(always)]
pub(super) unsafe fn deallocate<A, V>(allocator: &A, ptr: NonNull<V>)
where
    A: Allocator,
{
    let layout = Layout::new::<V>();

    unsafe { allocator.deallocate(ptr.cast()) }
}

pub struct BackendBox {
    ptr: NonNull<dyn Backend + 'static>,
    layout: Layout,
}

impl BackendBox {
    #[inline]
    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn new_in<A, B>(allocator: &A, value: B) -> Self
    where
        A: Allocator,
        B: Backend + 'static,
    {
        let layout = Layout::new::<B>();

        let Ok(ptr) = allocator.allocate(layout).map(NonNull::<_>::cast::<B>) else {
            handle_alloc_error(layout);
        };

        unsafe { ptr.write(value) };

        Self { ptr, layout }
    }

    #[inline]
    pub fn drop<A>(&self, allocator: &A)
    where
        A: Allocator,
    {
        unsafe {
            allocator.deallocate(self.ptr.cast());
        }
    }
}

impl Deref for BackendBox {
    type Target = dyn Backend + 'static;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl DerefMut for BackendBox {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}
