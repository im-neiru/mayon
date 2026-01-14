use core::alloc::Allocator;

use crate::{BaseError, backends::CreateError, logger::Logger};

pub trait Backend {}

pub trait CreateBackend<'s, A, L>
where
    A: Allocator + 'static,
    L: Logger + 'static,
    Self::Error: BaseError,
    Self: Backend,
{
    type Error;
    type Params;

    fn create(
        allocator: &A,
        logger: &mut L,
        params: Self::Params,
    ) -> Result<Self, CreateError<<Self::Error as BaseError>::ErrorKind>>
    where
        Self: Sized;
}
