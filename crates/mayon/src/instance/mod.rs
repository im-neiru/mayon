mod alloc;
mod inner;
pub mod logger;

use core::alloc::Allocator;
use std::alloc::Global;

use crate::backends::{Backend, CreateBackend};

use inner::ArcInner;

#[derive(Clone)]
pub struct Instance<A, L>(ArcInner<A, L>)
where
    A: Allocator + 'static,
    L: logger::Logger + 'static;

impl<A, L> Instance<A, L>
where
    A: Allocator + 'static,
    L: logger::Logger + 'static,
{
    pub fn new_in<'s, B>(params: B::Params, allocator: A, logger: L) -> Result<Self, B::Error>
    where
        B: Backend + CreateBackend<'s, A> + 'static,
    {
        let arc = ArcInner::new::<'s, B>(allocator, logger, params)?;

        Ok(Self(arc))
    }
}

impl<L> Instance<Global, L>
where
    L: logger::Logger + 'static,
{
    #[inline]
    pub fn new<'s, B>(params: B::Params, logger: L) -> Result<Self, B::Error>
    where
        B: Backend + CreateBackend<'s, Global> + 'static,
    {
        Self::new_in::<B>(params, Global, logger)
    }
}
