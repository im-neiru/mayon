use core::mem::transmute;

use allocator::Allocator;
use mayon::{Backend, Instance, logger::Logger};

impl<A, L, B> From<Instance<A, L, B>> for crate::MynInstance
where
    A: Allocator,
    L: Logger,
    B: Backend,
{
    #[inline(always)]
    fn from(value: Instance<A, L, B>) -> Self {
        unsafe { transmute::<Instance<A, L, B>, Self>(value) }
    }
}

impl crate::MynInstance {
    #[inline(always)]
    pub fn inner<A, L, B>(&self) -> &Instance<A, L, B>
    where
        A: Allocator,
        L: Logger,
        B: Backend,
    {
        let ptr: *const Instance<A, L, B> = (self as *const Self).cast();

        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    #[inline(always)]
    pub fn inner_mut<A, L, B>(&mut self) -> &mut Instance<A, L, B>
    where
        A: Allocator,
        L: Logger,
        B: Backend,
    {
        let ptr: *mut Instance<A, L, B> = (self as *mut Self).cast();

        unsafe { ptr.as_mut().unwrap_unchecked() }
    }
}
