use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use allocator::Allocator;

use crate::{BaseError, CreateBackendError, InstanceRef, context::ContextHandler, logger::Logger};

pub trait Backend {
    type Context: ContextHandler;
}

pub trait CreateContextFromRwh<L, A>: Backend
where
    A: Allocator,
    L: Logger,
    Self: Sized,
{
    fn create_context_from_rwh<H>(
        instance: &mut InstanceRef<Self, L, A>,
        handle: &H,
    ) -> Self::Context
    where
        H: HasDisplayHandle + HasWindowHandle;
}

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
