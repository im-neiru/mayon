use crate::backends::Backend;
use core::{
    alloc::{Allocator, Layout},
    ptr::NonNull,
};
use std::alloc::handle_alloc_error;
use std::ops::{Deref, DerefMut};

#[inline(always)]
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

    unsafe { allocator.deallocate(ptr.cast(), layout) }
}

pub struct BackendBox {
    ptr: NonNull<dyn Backend + 'static>,
    layout: Layout,
}

impl BackendBox {
    #[inline]
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
    pub fn drop<A>(&self, allocator: A)
    where
        A: Allocator,
    {
        unsafe {
            allocator.deallocate(self.ptr.cast(), self.layout);
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
