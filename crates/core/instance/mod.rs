mod inner;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use allocator::{Allocator, System};

use crate::{
    Backend, BaseError, CreateBackend, CreateBackendError, CreateContextErrorKind,
    CreateContextFromRwh,
    logger::{DefaultLogger, Logger},
};

use inner::ArcInner;

pub use inner::InstanceRef;

/// A Mayon instance, representing an initialized graphics backend.
///
/// The `Instance` is the primary entry point for the Mayon library. It maintains the lifetime
/// of the chosen graphics backend (e.g., Vulkan, DirectX) and provides methods to create
/// surfaces and rendering contexts.
///
/// It is internally reference-counted, making it cheap to clone, though clones still point
/// to the same underlying backend instance.
///
/// # Type Parameters
///
/// * `B`: The backend implementation (must implement [`Backend`]).
/// * `L`: The logger implementation for diagnostic output. Defaults to [`DefaultLogger`].
/// * `A`: The allocator used for internal memory management. Defaults to [`System`].
#[repr(transparent)]
pub struct Instance<B, L = DefaultLogger, A = System>(#[allow(unused)] ArcInner<B, L, A>)
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
    /// Creates a new `Instance` with a specific allocator and logger.
    ///
    /// This is the most flexible way to create an instance, allowing full control over
    /// memory allocation and logging.
    ///
    /// # Parameters
    ///
    /// * `params`: Backend-specific creation parameters.
    /// * `logger`: The logger to be used by the instance and all objects created from it.
    /// * `allocator`: The allocator to be used for all internal allocations.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateBackendError`] if the backend initialization fails.
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
    /// Creates a new `Instance` using the default system allocator.
    ///
    /// This is a convenience method for cases where custom allocation is not required.
    ///
    /// # Parameters
    ///
    /// * `params`: Backend-specific creation parameters.
    /// * `logger`: The logger to be used by the instance.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateBackendError`] if the backend initialization fails.
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
    /// Creates a new [`Context`](crate::Context) from raw window handles.
    ///
    /// This method allows Mayon to interface with externally created windows (e.g., from `winit`).
    ///
    /// # Parameters
    ///
    /// * `handle`: An object implementing both [`HasDisplayHandle`] and [`HasWindowHandle`].
    ///
    /// # Errors
    ///
    /// Returns a [`CreateContextError`](crate::CreateContextError) if the context could not be created.
    /// This can happen due to incompatible window handles, device loss, or allocation failures.
    #[allow(clippy::type_complexity)]
    pub fn create_context_from_rwh<H>(
        &mut self,
        handle: &H,
    ) -> Result<
        crate::Context<B, L, A>,
        crate::CreateContextError<<B::Error as BaseError>::ErrorKind>,
    >
    where
        B: CreateContextFromRwh<L, A>,
        B::Context: crate::context::DestroyContext<B, L, A>,
        H: HasDisplayHandle + HasWindowHandle,
    {
        let instance = self.create_ref();

        let Ok(context) =
            crate::Context::create(instance, B::create_context_from_rwh(instance, handle)?)
        else {
            return CreateContextErrorKind::AllocationFailed.into_result();
        };

        Ok(context)
    }
}
