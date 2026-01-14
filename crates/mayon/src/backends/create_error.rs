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

impl<B> CreateError<B>
where
    B: Copy + Clone + Debug + Display,
{
    #[inline]
    pub const fn kind(&self) -> CreateErrorKind<B> {
        self.kind
    }

    #[cfg(feature = "error_location")]
    #[inline]
    pub const fn location(&self) -> &'static Location<'static> {
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
