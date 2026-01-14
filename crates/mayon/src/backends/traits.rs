use std::alloc::Allocator;

use crate::logger::Logger;

pub trait Backend {}

pub trait CreateBackend<'s, A, L>
where
    A: Allocator + 'static,
    L: Logger + 'static,
    Self: Backend,
{
    type Error;
    type Params;

    fn create(allocator: &A, logger: &mut L, params: Self::Params) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
