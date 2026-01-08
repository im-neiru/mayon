use core::{alloc::Allocator, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicUsize};

use crate::backends::Backend;

use super::alloc::store_zeroed;

pub(crate) struct Inner<A>
where
    A: Allocator,
{
    allocator: A,
    backend: NonNull<dyn Backend>,

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
            let backend = store_zeroed(&allocator, backend);

            let mut buffer = store_zeroed(&allocator, MaybeUninit::<Inner<A>>::uninit());

            buffer.as_mut().write(Inner {
                allocator,
                backend,
                ref_count: AtomicUsize::new(1),
            });

            Self(backend.cast())
        }
    }
}
