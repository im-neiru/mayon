mod inner;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use allocator::{Allocator, System};

use crate::{
    Backend, BaseError, ContextHandler, CreateBackend, CreateBackendError, CreateContextFromRwh,
    logger::{DefaultLogger, Logger},
};

use inner::ArcInner;

pub use inner::InstanceRef;

pub struct Instance<B, L = DefaultLogger, A = System>(ArcInner<B, L, A>)
where
    B: Backend,
    L: Logger,
    A: Allocator;

impl<B, L, A> Instance<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    /// Creates an `Instance` by constructing backend `B` with the provided parameters, allocator, and logger.
    ///
    /// # Parameters
    /// - `params`: Backend-specific creation parameters.
    /// - `allocator`: Allocator to use for the instance.
    /// - `logger`: Logger to attach to the backend.
    ///
    /// # Returns
    /// `Ok(Self)` containing the created instance, or `Err(CreateBackendError<<B::Error as BaseError>::ErrorKind>)` if backend creation fails.
    /// ```
    pub fn new_in<'s>(
        params: B::Params,
        logger: L,
        allocator: A,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>>
    where
        B: CreateBackend<'s, A, L>,
    {
        let arc = ArcInner::new(allocator, logger, params)?;

        Ok(Self(arc))
    }
}

impl<B, L> Instance<B, L, System>
where
    B: Backend,
    L: Logger,
{
    /// Creates a new Instance using the global allocator for backend `B`.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if backend creation succeeds, `Err(CreateBackendError<<B::Error as BaseError>::ErrorKind>)` otherwise.
    #[inline]
    pub fn new<'s>(
        params: B::Params,
        logger: L,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>>
    where
        B: CreateBackend<'s, System, L>,
    {
        Self::new_in(params, logger, System)
    }
}

impl<B, L, A> Instance<B, L, A>
where
    B: Backend,
    L: Logger,
    A: Allocator,
{
    pub fn create_context_from_rwh<H>(&mut self, handle: &H) -> B::Context
    where
        B: CreateContextFromRwh<L, A>,
        H: HasDisplayHandle + HasWindowHandle,
    {
        let instance = self.create_ref();

        B::create_context_from_rwh(instance, handle)
    }
}
