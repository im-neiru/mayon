use core::{
    fmt::{Debug, Display},
    panic::Location,
};

#[derive(Copy, Clone, Debug, thiserror::Error)]
#[error("{kind}")]
pub struct CreateContextError<B>
where
    B: Copy + Clone + Debug + Display,
{
    pub(crate) kind: CreateContextErrorKind<B>,
    #[cfg(feature = "error_location")]
    pub(crate) location: &'static Location<'static>,
}

impl<B> CreateContextError<B>
where
    B: Copy + Clone + Debug + Display,
{
    /// Creates a new `CreateContextError` with the given error kind.
    ///
    /// The `location` argument is included only when the `error_location` feature is enabled
    /// and captures the caller location for diagnostic purposes.
    pub const fn new(
        kind: CreateContextErrorKind<B>,
        #[cfg(feature = "error_location")] location: &'static Location<'static>,
    ) -> Self {
        Self {
            kind,
            #[cfg(feature = "error_location")]
            location,
        }
    }
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum CreateContextErrorKind<B>
where
    B: Copy + Clone + Debug + Display,
{
    #[error("Allocating memory for context failed")]
    AllocationFailed,

    #[error("Unsupported platform")]
    UnsupportedPlatform,

    #[error("{0}")]
    BackendInternal(B),
}

impl<B> crate::BaseError for CreateContextError<B>
where
    B: Copy + Clone + Debug + Display,
{
    type ErrorKind = CreateContextErrorKind<B>;

    /// Returns the error's kind.
    fn kind(&self) -> Self::ErrorKind {
        self.kind
    }

    /// Get the stored caller location associated with this error.
    #[cfg(feature = "error_location")]
    #[inline]
    fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

impl<B> CreateContextErrorKind<B>
where
    B: Copy + Clone + Debug + Display,
{
    /// Convert this `CreateContextErrorKind` into an error `Result`, producing a
    /// `CreateContextError` that captures the caller's source location.
    ///
    /// # Returns
    ///
    /// `Err(CreateContextError)` containing this kind and the call-site `Location`.
    #[cfg(feature = "error_location")]
    #[inline]
    #[track_caller]
    pub const fn into_result<T>(self) -> Result<T, self::CreateContextError<B>> {
        Err(CreateContextError {
            kind: self,
            location: Location::caller(),
        })
    }

    /// Converts this `CreateContextErrorKind` into an `Err` value containing a `CreateContextError`.
    ///
    /// This version is used when the `error_location` feature is disabled and therefore does not attach a call-site location to the error.
    ///
    /// # Returns
    ///
    /// `Err(CreateContextError)` with `kind` set to `self`.
    #[cfg(not(feature = "error_location"))]
    #[inline]
    pub const fn into_result<T>(self) -> Result<T, self::CreateContextError<B>> {
        Err(CreateContextError { kind: self })
    }
}
