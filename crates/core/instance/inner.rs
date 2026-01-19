use core::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering, fence},
};

use allocator::Allocator;

use crate::{
    Backend, BaseError, CreateBackend, CreateBackendError, CreateBackendErrorKind, logger::Logger,
};

pub(crate) struct Inner<A, L, B>
where
    A: Allocator,
    L: Logger,
    B: Backend,
{
    allocator: A,
    logger: L,
    backend: B,

    ref_count: AtomicUsize,
}

pub(crate) struct ArcInner<A, L, B>(NonNull<Inner<A, L, B>>)
where
    A: Allocator,
    L: Logger,
    B: Backend;

impl<'s, A, L, B> ArcInner<A, L, B>
where
    A: Allocator,
    L: Logger,
    B: Backend + CreateBackend<'s, A, L>,
{
    /// Allocates and initializes a new ArcInner containing the given allocator, logger, and a backend created from `params`.
    ///
    /// The returned ArcInner starts with a reference count of 1 and holds the backend produced by `B::create`.
    ///
    /// # Errors
    ///
    /// Returns a `CreateBackendError` if backend creation fails.
    pub(super) fn new(
        allocator: A,
        logger: L,
        params: B::Params,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>> {
        let Ok(mut buffer) = (unsafe { allocator.allocate_uninit::<Inner<A, L, B>>() }) else {
            return CreateBackendErrorKind::AllocationFailed.into_result();
        };

        unsafe {
            let inner = buffer.as_mut().assume_init_mut();

            inner.allocator = allocator;
            inner.logger = logger;
            inner.ref_count = AtomicUsize::new(1);
            inner.backend = B::create(&inner.allocator, &mut inner.logger, params)?;
        }

        Ok(Self(buffer.cast()))
    }

    // #[allow(unused)]
    // #[inline(always)]
    // pub(crate) fn backend(&self) -> &dyn Backend {
    //     unsafe { self.0.as_ref().backend.assume_init_ref().deref() }
    // }

    // #[allow(unused)]
    // #[inline(always)]
    // pub(crate) fn backend_mut(&mut self) -> &dyn Backend {
    //     unsafe { self.0.as_mut().backend.assume_init_mut().deref_mut() }
    // }
}

impl<A, L, B> Clone for ArcInner<A, L, B>
where
    A: Allocator,
    L: Logger,
    B: Backend,
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

impl<A, L, B> Drop for ArcInner<A, L, B>
where
    A: Allocator,
    L: Logger,
    B: Backend,
{
    fn drop(&mut self) {
        unsafe {
            if self.0.as_ref().ref_count.fetch_sub(1, Ordering::Release) != 1 {
                return;
            }

            fence(Ordering::Acquire);

            let Self(this) = self;
            let allocator = &this.as_ref().allocator;

            allocator.deallocate_init(*this);
        }
    }
}

unsafe impl<A, L, B> Send for ArcInner<A, L, B>
where
    A: Allocator,
    L: Logger,
    B: Backend,
{
}
unsafe impl<A, L, B> Sync for ArcInner<A, L, B>
where
    A: Allocator,
    L: Logger,
    B: Backend,
{
}
