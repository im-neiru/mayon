use core::mem::transmute;

use allocator::Allocator;
use mayon::{Backend, Instance, logger::Logger};

impl<B, L, A> From<Instance<B, L, A>> for crate::MynInstance
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    #[inline(always)]
    fn from(value: Instance<B, L, A>) -> Self {
        unsafe { transmute::<Instance<B, L, A>, Self>(value) }
    }
}

impl crate::MynInstance {
    #[inline(always)]
    pub fn inner<B, L, A>(&self) -> &Instance<B, L, A>
    where
        B: Backend,
        L: Logger,
        A: Allocator,
    {
        let ptr: *const Instance<B, L, A> = (self as *const Self).cast();

        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    #[inline(always)]
    pub fn inner_mut<B, L, A>(&mut self) -> &mut Instance<B, L, A>
    where
        B: Backend,
        L: Logger,
        A: Allocator,
    {
        let ptr: *mut Instance<B, L, A> = (self as *mut Self).cast();

        unsafe { ptr.as_mut().unwrap_unchecked() }
    }
}
