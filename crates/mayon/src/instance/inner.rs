use core::{
    alloc::Allocator,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering, fence},
};

use crate::backends::{Backend, CreateBackend};

use super::alloc::{BackendBox, allocate, deallocate};

pub(crate) struct Inner<A>
where
    A: Allocator,
{
    allocator: A,
    backend: MaybeUninit<BackendBox>,

    ref_count: AtomicUsize,
}

pub(crate) struct ArcInner<A>(NonNull<Inner<A>>)
where
    A: Allocator;

impl<A> ArcInner<A>
where
    A: Allocator,
{
    pub(super) fn new<'s, B>(allocator: A, params: B::Params) -> Result<Self, B::Error>
    where
        B: Backend + CreateBackend<'s, A> + Send + Sync + 'static,
    {
        unsafe {
            let mut buffer = allocate(&allocator, MaybeUninit::<Inner<A>>::uninit());

            buffer.as_mut().write(Inner {
                allocator,
                backend: MaybeUninit::uninit(),
                ref_count: AtomicUsize::new(1),
            });

            let backend = BackendBox::new_in(
                &buffer.as_ref().assume_init_ref().allocator,
                B::create(&buffer.as_ref().assume_init_ref().allocator, params)?,
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

impl<A> Clone for ArcInner<A>
where
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

impl<A> Drop for ArcInner<A>
where
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

            this.as_ref().backend.assume_init_ref().drop(allocator);

            deallocate(allocator, *this);
        }
    }
}

unsafe impl<A: Allocator + Send> Send for ArcInner<A> {}
unsafe impl<A: Allocator + Sync> Sync for ArcInner<A> {}
