use core::{
    fmt::{Debug, Display},
    panic::Location,
};

#[derive(Copy, Clone, Debug, thiserror::Error)]
#[error("{kind}")]
pub struct CreateError<B>
where
    B: Copy + Clone + Debug + Display,
{
    pub(crate) kind: CreateErrorKind<B>,
    #[cfg(feature = "error_location")]
    pub(crate) location: &'static Location<'static>,
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum CreateErrorKind<B>
where
    B: Copy + Clone + Debug + Display,
{
    #[error("Unsupported Target Window Platform")]
    UnsupportedTargetPlatform,
    #[error("{0}")]
    BackendInternal(B),
}

impl<B> crate::HasErrorKind for CreateError<B>
where
    B: Copy + Clone + Debug + Display,
{
    type ErrorKind = CreateErrorKind<B>;

    fn kind(&self) -> Self::ErrorKind {
        self.kind
    }
}

impl<B> crate::HasErrorLocation for CreateError<B>
where
    B: Copy + Clone + Debug + Display,
{
    #[cfg(feature = "error_location")]
    #[inline]
    fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

impl<B> CreateErrorKind<B>
where
    B: Copy + Clone + Debug + Display,
{
    #[cfg(feature = "error_location")]
    #[inline]
    #[track_caller]
    pub(super) const fn into_result<T>(self) -> Result<T, self::CreateError<B>> {
        Err(CreateError {
            kind: self,
            location: Location::caller(),
        })
    }

    #[cfg(not(feature = "error_location"))]
    #[inline]
    pub(super) const fn into_result<T>(self) -> self::Result<T> {
        Err(CreateError { kind: self })
    }
}
