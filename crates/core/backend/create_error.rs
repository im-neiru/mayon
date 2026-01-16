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

    fn kind(&self) -> Self::ErrorKind {
        self.kind
    }

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
    #[cfg(feature = "error_location")]
    #[inline]
    #[track_caller]
    pub const fn into_result<T>(self) -> Result<T, self::CreateBackendError<B>> {
        Err(CreateBackendError {
            kind: self,
            location: Location::caller(),
        })
    }

    #[cfg(not(feature = "error_location"))]
    #[inline]
    pub const fn into_result<T>(self) -> self::Result<T> {
        Err(CreateBackendError { kind: self })
    }
}
