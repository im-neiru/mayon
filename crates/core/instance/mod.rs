mod alloc;
mod inner;

use core::alloc::Allocator;
use std::alloc::Global;

use crate::{Backend, BaseError, CreateBackend, CreateBackendError, logger::Logger};

use inner::ArcInner;

#[derive(Clone)]
pub struct Instance<A, L>(ArcInner<A, L>)
where
    A: Allocator + 'static,
    L: Logger + 'static;

impl<A, L> Instance<A, L>
where
    A: Allocator + 'static,
    L: Logger + 'static,
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
    pub fn new_in<'s, B>(
        params: B::Params,
        allocator: A,
        logger: L,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>>
    where
        B: Backend + CreateBackend<'s, A, L> + 'static,
    {
        let arc = ArcInner::new::<'s, B>(allocator, logger, params)?;

        Ok(Self(arc))
    }
}

impl<L> Instance<Global, L>
where
    L: Logger + 'static,
{
    /// Creates a new Instance using the global allocator for backend `B`.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if backend creation succeeds, `Err(CreateBackendError<<B::Error as BaseError>::ErrorKind>)` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let instance = Instance::new::<'static, VulkanBackend>(
    ///     VulkanBackendParams::default()
    ///         .with_application_name(c"Mayon")
    ///         .with_engine_name(c"Mayon Engine")
    ///         .with_application_version((1, 0)),
    ///     DefaultLogger
    /// ).unwrap();
    /// ```
    #[inline]
    pub fn new<'s, B>(
        params: B::Params,
        logger: L,
    ) -> Result<Self, CreateBackendError<<B::Error as BaseError>::ErrorKind>>
    where
        B: Backend + CreateBackend<'s, Global, L> + 'static,
    {
        Self::new_in::<B>(params, Global, logger)
    }
}
