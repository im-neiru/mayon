use crate::backends::Backend;
use core::{alloc::Allocator, ptr::NonNull, sync::atomic::AtomicUsize};
use std::mem::MaybeUninit;

pub(crate) struct Inner<A>
where
    A: Allocator,
{
    allocator: A,
    backend: NonNull<dyn Backend>,

    ref_count: AtomicUsize,
}

impl<A> Inner<A>
where
    A: Allocator,
{
    pub(super) unsafe fn new<B>(allocator: A, backend: B) -> NonNull<Self>
    where
        B: Backend + 'static,
    {
        unsafe {
            let backend = Self::store_zeroed(&allocator, backend);

            let mut buffer = Self::store_zeroed(&allocator, MaybeUninit::<Self>::uninit());

            buffer.as_mut().write(Self {
                allocator,
                backend,
                ref_count: AtomicUsize::new(1),
            });

            backend.cast()
        }
    }
}
