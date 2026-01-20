mod create_error;

use core::{ops, ptr::NonNull};

use crate::{Backend, InstanceRef, logger::Logger};
use allocator::{AllocError, Allocator};

pub use create_error::{CreateContextError, CreateContextErrorKind};

pub trait ContextHandler {}

pub trait DestroyContext<B, L, A>: ContextHandler
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    fn destroy(&self, instance: &InstanceRef<B, L, A>);
}

pub struct Context<B, L, A>(NonNull<Inner<B, L, A>>)
where
    B: Backend,
    B::Context: DestroyContext<B, L, A>,
    L: Logger,
    A: Allocator;
struct Inner<B, L, A>
where
    B: Backend,
    B::Context: DestroyContext<B, L, A>,
    L: Logger,
    A: Allocator,
{
    instance: InstanceRef<B, L, A>,
    context: B::Context,
}

impl<B, L, A> Context<B, L, A>
where
    B: Backend,
    B::Context: DestroyContext<B, L, A>,
    L: Logger,
    A: Allocator,
{
    #[inline]
    pub(crate) fn create(
        instance: &InstanceRef<B, L, A>,
        context: B::Context,
    ) -> Result<Self, AllocError> {
        let Ok(ptr) = (unsafe {
            instance.allocator().allocate_init(Inner {
                instance: instance.clone(),
                context,
            })
        }) else {
            return Err(AllocError);
        };

        Ok(Self(ptr))
    }
}

impl<B, L, A> ops::Deref for Context<B, L, A>
where
    B: Backend,
    B::Context: DestroyContext<B, L, A>,
    L: Logger,
    A: Allocator,
{
    type Target = B::Context;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.as_ref().context }
    }
}

impl<B, L, A> Drop for Context<B, L, A>
where
    B: Backend,
    B::Context: DestroyContext<B, L, A>,
    L: Logger,
    A: Allocator,
{
    fn drop(&mut self) {
        let inner = unsafe { self.0.as_mut() };
        let allocator = inner.instance.allocator();

        inner.context.destroy(&inner.instance);

        unsafe { allocator.deallocate_init::<Inner<B, L, A>>(self.0) }
    }
}
