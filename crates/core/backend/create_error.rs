use core::{
    fmt::{Debug, Display},
    panic::Location,
};

#[derive(Copy, Clone, Debug, thiserror::Error)]
#[error("{kind}")]
pub struct CreateBackendError<B>
where
    B: Copy + Clone + Debug + Display,
{
    pub(crate) kind: BackendCreateKind<B>,
    #[cfg(feature = "error_location")]
    pub(crate) location: &'static Location<'static>,
}

impl<B> CreateBackendError<B>
where
    B: Copy + Clone + Debug + Display,
{
    /// Creates a new `CreateBackendError` with the given error kind.
    ///
    /// The `location` argument is included only when the `error_location` feature is enabled
    /// and captures the caller location for diagnostic purposes.
    pub const fn new(
        kind: BackendCreateKind<B>,
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
pub enum BackendCreateKind<B>
where
    B: Copy + Clone + Debug + Display,
{
    #[error("Unsupported Target Window Platform")]
    UnsupportedTargetPlatform,
    #[error("{0}")]
    BackendInternal(B),
}

impl<B> crate::BaseError for CreateBackendError<B>
where
    B: Copy + Clone + Debug + Display,
{
    type ErrorKind = BackendCreateKind<B>;

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

impl<B> BackendCreateKind<B>
where
    B: Copy + Clone + Debug + Display,
{
    /// Convert this `BackendCreateKind` into an error `Result`, producing a
    /// `CreateBackendError` that captures the caller's source location.
    ///
    /// # Returns
    ///
    /// `Err(CreateBackendError)` containing this kind and the call-site `Location`.
    #[cfg(feature = "error_location")]
    #[inline]
    #[track_caller]
    pub const fn into_result<T>(self) -> Result<T, self::CreateBackendError<B>> {
        Err(CreateBackendError {
            kind: self,
            location: Location::caller(),
        })
    }

    /// Converts this `BackendCreateKind` into an `Err` value containing a `CreateBackendError`.
    ///
    /// This version is used when the `error_location` feature is disabled and therefore does not attach a call-site location to the error.
    ///
    /// # Returns
    ///
    /// `Err(CreateBackendError)` with `kind` set to `self`.
    #[cfg(not(feature = "error_location"))]
    #[inline]
    pub const fn into_result<T>(self) -> self::Result<T> {
        Err(CreateBackendError { kind: self })
    }
}
