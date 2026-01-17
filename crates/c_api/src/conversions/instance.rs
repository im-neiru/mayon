use core::mem::transmute;

use allocator::Allocator;
use mayon::{Instance, logger::Logger};

impl<A, L> From<Instance<A, L>> for crate::MynInstance
where
    A: Allocator,
    L: Logger,
{
    #[inline(always)]
    fn from(value: Instance<A, L>) -> Self {
        unsafe { transmute::<Instance<A, L>, Self>(value) }
    }
}

impl crate::MynInstance {
    #[inline(always)]
    pub fn inner<A, L>(&self) -> &Instance<A, L>
    where
        A: Allocator + 'static,
        L: Logger + 'static,
    {
        let ptr: *const Instance<A, L> = (self as *const Self).cast();

        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    #[inline(always)]
    pub fn inner_mut<A, L>(&mut self) -> &mut Instance<A, L>
    where
        A: Allocator + 'static,
        L: Logger + 'static,
    {
        let ptr: *mut Instance<A, L> = (self as *mut Self).cast();

        unsafe { ptr.as_mut().unwrap_unchecked() }
    }
}
