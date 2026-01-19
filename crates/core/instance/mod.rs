mod inner;

use allocator::{Allocator, System};

use crate::{Backend, BaseError, CreateBackend, CreateBackendError, logger::Logger};

use inner::ArcInner;

#[derive(Clone)]
pub struct Instance<A, L, B>(#[allow(unused)] ArcInner<A, L, B>)
where
    A: Allocator,
    L: Logger,
    B: Backend;

impl<'s, A, L, B> Instance<A, L, B>
where
    A: Allocator,
    L: Logger,
    B: Backend + CreateBackend<'s, A, L>,
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
    pub fn new_in(
        params: B::Params,
        allocator: A,
        logger: L,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>> {
        let arc = ArcInner::new(allocator, logger, params)?;

        Ok(Self(arc))
    }
}

impl<'s, L, B> Instance<System, L, B>
where
    L: Logger,
    B: Backend + CreateBackend<'s, System, L>,
{
    /// Creates a new Instance using the global allocator for backend `B`.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if backend creation succeeds, `Err(CreateBackendError<<B::Error as BaseError>::ErrorKind>)` otherwise.
    #[inline]
    pub fn new(
        params: B::Params,
        logger: L,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>> {
        Self::new_in(params, System, logger)
    }
}
