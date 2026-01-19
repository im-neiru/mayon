use allocator::Allocator;

use crate::{BaseError, CreateBackendError, logger::Logger};

pub trait Backend {}

pub trait CreateBackend<'s, A, L>
where
    A: Allocator,
    L: Logger,
    Self::Error: BaseError,
    Self: Backend,
{
    type Error;
    type Params;

    fn create(
        allocator: &A,
        logger: &mut L,
        params: Self::Params,
    ) -> Result<Self, CreateBackendError<<Self::Error as BaseError>::ErrorKind>>
    where
        Self: Sized;
}
