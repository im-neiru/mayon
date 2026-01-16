use core::{
    alloc::Allocator,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering, fence},
};

use crate::{Backend, BaseError, CreateBackend, CreateBackendError};

use super::{
    alloc::{BackendBox, allocate, deallocate},
    logger::Logger,
};

pub(crate) struct Inner<A, L>
where
    A: Allocator,
    L: Logger,
{
    allocator: A,
    logger: L,
    backend: MaybeUninit<BackendBox>,

    ref_count: AtomicUsize,
}

pub(crate) struct ArcInner<A, L>(NonNull<Inner<A, L>>)
where
    A: Allocator,
    L: Logger;

impl<A, L> ArcInner<A, L>
where
    A: Allocator + 'static,
    L: Logger + 'static,
{
    pub(super) fn new<'s, B>(
        allocator: A,
        logger: L,
        params: B::Params,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>>
    where
        B: Backend + CreateBackend<'s, A, L> + 'static,
    {
        unsafe {
            let mut buffer = allocate(&allocator, MaybeUninit::<Inner<A, L>>::uninit());

            buffer.as_mut().write(Inner {
                allocator,
                logger,
                backend: MaybeUninit::uninit(),
                ref_count: AtomicUsize::new(1),
            });

            let inner = buffer.as_mut().assume_init_mut();

            let backend = BackendBox::new_in(
                &inner.allocator,
                B::create(&inner.allocator, &mut inner.logger, params)?,
            );

            buffer.as_mut().assume_init_mut().backend = MaybeUninit::new(backend);

            Ok(Self(buffer.cast()))
        }
    }

    #[inline(always)]
    pub(crate) fn backend(&self) -> &dyn Backend {
        unsafe { self.0.as_ref().backend.assume_init_ref().deref() }
    }

    #[inline(always)]
    pub(crate) fn backend_mut(&mut self) -> &dyn Backend {
        unsafe { self.0.as_mut().backend.assume_init_mut().deref_mut() }
    }
}

impl<A, L> Clone for ArcInner<A, L>
where
    A: Allocator,
    L: Logger,
{
    fn clone(&self) -> Self {
        const MAX_REFCOUNT: usize = (isize::MAX) as _;

        let old_count = unsafe { self.0.as_ref() }
            .ref_count
            .fetch_add(1, Ordering::Relaxed);

        if old_count >= MAX_REFCOUNT {
            std::process::abort();
        }

        Self(self.0)
    }
}

impl<A, L> Drop for ArcInner<A, L>
where
    A: Allocator,
    L: Logger,
{
    fn drop(&mut self) {
        unsafe {
            if self.0.as_ref().ref_count.fetch_sub(1, Ordering::Release) != 1 {
                return;
            }

            fence(Ordering::Acquire);

            let Self(this) = self;
            let allocator = &this.as_ref().allocator;

            this.as_ref().backend.assume_init_ref().drop(allocator);

            deallocate(allocator, *this);
        }
    }
}

unsafe impl<A: Allocator + Send, L: Logger> Send for ArcInner<A, L> {}
unsafe impl<A: Allocator + Sync, L: Logger> Sync for ArcInner<A, L> {}
