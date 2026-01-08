use core::panic::Location;

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

impl Error {
    #[inline]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[cfg(feature = "error_location")]
    #[inline]
    pub const fn location(&self) -> &'static Location<'static> {
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
