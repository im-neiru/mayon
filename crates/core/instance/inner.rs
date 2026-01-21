use core::{
    mem::{offset_of, transmute},
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering, fence},
};

use allocator::Allocator;

use crate::{
    Backend, BaseError, CreateBackend, CreateBackendError, CreateBackendErrorKind, logger::Logger,
};

pub(crate) struct Inner<B, L, A>
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

#[repr(transparent)]
pub(crate) struct ArcInner<B, L, A>(NonNull<Inner<B, L, A>>)
where
    B: Backend,
    A: Allocator,
    L: Logger;

impl<B, L, A> ArcInner<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    /// Allocates and initializes a new ArcInner containing the given allocator, logger, and a backend created from `params`.
    ///
    /// The returned ArcInner starts with a reference count of 1 and holds the backend produced by `B::create`.
    ///
    /// # Errors
    ///
    /// Returns a `CreateBackendError` if backend creation fails.
    pub(super) fn new<'s>(
        allocator: A,
        logger: L,
        params: B::Params,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>>
    where
        B: CreateBackend<'s, A, L>,
    {
        let Ok(mut buffer) = (unsafe { allocator.allocate_uninit::<Inner<B, L, A>>() }) else {
            return CreateBackendErrorKind::AllocationFailed.into_result();
        };

        unsafe {
            buffer
                .byte_add(offset_of!(Inner<B, L, A>, allocator))
                .cast()
                .write(allocator);

            buffer
                .byte_add(offset_of!(Inner<B, L, A>, logger))
                .cast()
                .write(logger);

            buffer
                .byte_add(offset_of!(Inner<B, L, A>, ref_count))
                .cast()
                .write(AtomicUsize::new(1));

            buffer
                .byte_add(offset_of!(Inner<B, L, A>, backend))
                .cast()
                .write(B::create(
                    &buffer.as_ref().assume_init_ref().allocator,
                    &mut buffer.as_mut().assume_init_mut().logger,
                    params,
                )?);
        }

        Ok(Self(buffer.cast()))
    }
}

impl<B, L, A> Clone for ArcInner<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
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

impl<B, L, A> Drop for ArcInner<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
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

unsafe impl<B, L, A> Send for ArcInner<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
}
unsafe impl<B, L, A> Sync for ArcInner<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
}

/// A shared reference to a Mayon [`Instance`].
///
/// `InstanceRef` provides read-only access to the underlying backend, logger, and allocator
/// of a Mayon instance. It is often passed to backend-specific functions that require
/// access to the global state.
///
/// Like [`Instance`], it is internally reference-counted and cheap to clone.
#[repr(transparent)]
pub struct InstanceRef<B, L, A>(ArcInner<B, L, A>)
where
    B: Backend,
    L: Logger,
    A: Allocator;

impl<B, L, A> InstanceRef<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    /// Returns a reference to the graphics backend.
    #[inline(always)]
    pub fn backend(&self) -> &B {
        unsafe { &self.0.0.as_ref().backend }
    }

    /// Returns a reference to the logger associated with this instance.
    #[inline(always)]
    pub fn logger(&self) -> &L {
        unsafe { &self.0.0.as_ref().logger }
    }

    /// Returns a reference to the allocator used by this instance.
    #[inline(always)]
    pub fn allocator(&self) -> &A {
        unsafe { &self.0.0.as_ref().allocator }
    }
}

impl<B, L, A> Clone for InstanceRef<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<B, L, A> crate::Instance<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    #[inline(always)]
    pub(crate) fn create_ref(&mut self) -> &mut InstanceRef<B, L, A> {
        unsafe { transmute::<&mut Self, &mut InstanceRef<B, L, A>>(self) }
    }
}
