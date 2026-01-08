use std::alloc::Allocator;

pub trait Backend {}

pub trait CreateBackend<'s, A>
where
    A: Allocator + 'static,
    Self: Backend,
{
    type Error;
    type Params;

    fn create(allocator: &A, params: Self::Params) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
