mod alloc;
mod inner;

use core::alloc::Allocator;
use std::alloc::Global;

use crate::backends::{Backend, CreateBackend};

use inner::ArcInner;

#[derive(Clone)]
pub struct Instance<A: Allocator = Global>(ArcInner<A>)
where
    A: Allocator;

impl<A: Allocator> Instance<A>
where
    A: Allocator + 'static,
{
    pub fn new_in<'s, B>(params: B::Params, allocator: A) -> Result<Self, B::Error>
    where
        B: Backend + CreateBackend<'s, A> + 'static,
    {
        let arc = ArcInner::new::<'s, B>(allocator, params)?;

        Ok(Self(arc))
    }
}

impl Instance<Global> {
    #[inline]
    pub fn new<'s, B>(params: B::Params) -> Result<Self, B::Error>
    where
        B: Backend + CreateBackend<'s, Global> + 'static,
    {
        Self::new_in::<B>(params, Global)
    }
}
