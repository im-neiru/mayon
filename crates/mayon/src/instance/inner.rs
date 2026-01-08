use crate::backends::Backend;
use core::{alloc::Allocator, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;

use super::alloc::{BackendBox, allocate, deallocate};

pub(crate) struct Inner<A>
where
    A: Allocator,
{
    allocator: A,
    backend: BackendBox,

    ref_count: AtomicUsize,
}

pub(crate) struct ArcInner<A>(NonNull<Inner<A>>)
where
    A: Allocator;

impl<A> ArcInner<A>
where
    A: Allocator,
{
    pub(super) unsafe fn new<B>(allocator: A, backend: B) -> Self
    where
        B: Backend + 'static,
    {
        unsafe {
            let backend = BackendBox::new_in(&allocator, backend);

            let mut buffer = allocate(&allocator, MaybeUninit::<Inner<A>>::uninit());

            buffer.as_mut().write(Inner {
                allocator,
                backend,
                ref_count: AtomicUsize::new(1),
            });

            Self(buffer.cast())
        }
    }
}

impl<A> Clone for ArcInner<A>
where
    A: Allocator,
{
    fn clone(&self) -> Self {
        const MAX_REFCOUNT: usize = (isize::MAX) as _;

        let old_count = unsafe { self.0.as_ref() }.ref_count.fetch_add(1, Relaxed);

        if old_count >= MAX_REFCOUNT {
            std::process::abort();
        }

        Self(self.0)
    }
}

impl<A> Drop for ArcInner<A>
where
    A: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            let Self(this) = self;
            let allocator = &this.as_ref().allocator;

            this.as_ref().backend.drop(allocator);

            deallocate(allocator, *this);
        }
    }
}
