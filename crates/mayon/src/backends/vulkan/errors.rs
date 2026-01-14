use core::panic::Location;

use crate::backends::{CreateError, CreateErrorKind::BackendInternal};

#[derive(Copy, Clone, Debug, thiserror::Error)]
#[error("{kind}")]
pub struct Error {
    pub(crate) kind: ErrorKind,
    #[cfg(feature = "error_location")]
    pub(crate) location: &'static Location<'static>,
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Failed to load Vulkan")]
    VulkanLoad,

    #[error("{function_name} failed: {code}")]
    VulkanFunctionError {
        function_name: &'static str,
        code: super::ReturnCode,
    },
}

pub type Result<T> = core::result::Result<T, Error>;

impl crate::BaseError for Error {
    type ErrorKind = ErrorKind;

    fn kind(&self) -> Self::ErrorKind {
        self.kind
    }

    #[cfg(feature = "error_location")]
    #[inline]
    fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

impl ErrorKind {
    #[cfg(feature = "error_location")]
    #[inline]
    #[track_caller]
    pub(super) const fn into_result<T>(self) -> self::Result<T> {
        Err(Error {
            kind: self,
            location: Location::caller(),
        })
    }

    #[cfg(not(feature = "error_location"))]
    #[inline]
    pub(super) const fn into_result<T>(self) -> self::Result<T> {
        Err(Error { kind: self })
    }
}

impl<'a> From<self::Error> for CreateError<self::ErrorKind> {
    fn from(value: self::Error) -> Self {
        Self {
            kind: BackendInternal(value.kind),
            location: value.location,
        }
    }
}
