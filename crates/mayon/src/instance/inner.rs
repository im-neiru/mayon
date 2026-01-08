use core::{alloc::Allocator, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicUsize};

use crate::backends::Backend;

use super::alloc::{BackendBox, allocate};

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

impl<A> Drop for ArcInner<A>
where
    A: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            let Self(this) = self;
            let allocator = &this.as_ref().allocator;

            this.as_ref().backend.drop(allocator)
        }
    }
}
