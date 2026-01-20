use core::{ops, ptr::NonNull};

use crate::{Backend, InstanceRef, logger::Logger};
use allocator::Allocator;

pub trait ContextHandler {}

pub struct Context<B, L, A>(NonNull<Inner<B, L, A>>)
where
    B: Backend,
    L: Logger,
    A: Allocator;
struct Inner<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    instance: InstanceRef<B, L, A>,
    context: B::Context,
}

impl<B, L, A> ops::Deref for Context<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    type Target = B::Context;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.as_ref().context }
    }
}

impl<B, L, A> ops::DerefMut for Context<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.0.as_mut().context }
    }
}

impl<B, L, A> Drop for Context<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    fn drop(&mut self) {
        let allocator = unsafe { self.0.as_ref().instance.allocator() };

        unsafe { allocator.deallocate_init::<Inner<B, L, A>>(self.0) }
    }
}
