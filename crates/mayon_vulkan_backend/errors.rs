use core::panic::Location;

use mayon_core::{BaseError, CreateBackendError, CreateBackendErrorKind::BackendInternal};

#[derive(Copy, Clone, Debug, thiserror::Error)]
#[error("{kind}")]
pub struct VulkanError {
    pub(crate) kind: VulkanErrorKind,
    #[cfg(feature = "error_location")]
    pub(crate) location: &'static Location<'static>,
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum VulkanErrorKind {
    #[error("Failed to load Vulkan library")]
    LibraryLoad,

    #[error("Failed to load function {name}")]
    FunctionLoadFailed { name: crate::VulkanFunctionName },

    #[error("{name} returned {code}")]
    FunctionReturn {
        name: crate::VulkanFunctionName,
        code: super::ReturnCode,
    },
}

pub type Result<T> = core::result::Result<T, VulkanError>;

impl BaseError for VulkanError {
    type ErrorKind = VulkanErrorKind;

    /// Accesses the error's kind.
    ///
    /// # Returns
    ///
    /// `ErrorKind` describing the category of this error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let err: Error = unimplemented!();
    /// let kind = err.kind();
    /// ```
    fn kind(&self) -> Self::ErrorKind {
        self.kind
    }

    /// Returns the stored source code location associated with this error.
    ///
    /// The returned reference points to the static `Location` captured when the error was created.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::panic::Location;
    ///
    /// struct S { location: &'static Location<'static> }
    /// impl S {
    ///     fn location(&self) -> &'static Location<'static> { self.location }
    /// }
    ///
    /// let s = S { location: Location::caller() };
    /// let loc = s.location();
    /// assert_eq!(loc, s.location());
    /// ```
    #[cfg(feature = "error_location")]
    #[inline]
    fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

impl VulkanErrorKind {
    #[cfg(feature = "error_location")]
    #[inline]
    #[track_caller]
    pub(super) const fn into_result<T>(self) -> self::Result<T> {
        Err(VulkanError {
            kind: self,
            location: Location::caller(),
        })
    }

    /// Convert an `ErrorKind` into a `Result` that is an `Err` containing the corresponding `Error`.
    ///
    /// # Returns
    ///
    /// An `Err` containing an `Error` constructed from this `ErrorKind`.
    ///
    /// # Examples
    ///
    /// ```
    /// // assuming `ErrorKind::VulkanLoad` is in scope and `Result` type alias is available
    /// let res: Result<i32> = ErrorKind::VulkanLoad.into_result();
    /// assert!(res.is_err());
    /// ```
    #[cfg(not(feature = "error_location"))]
    #[inline]
    pub(super) const fn into_result<T>(self) -> self::Result<T> {
        Err(VulkanError { kind: self })
    }
}

impl From<self::VulkanError> for CreateBackendError<self::VulkanErrorKind> {
    /// Converts a local `Error` into a `CreateBackendError` by wrapping the error's kind in `BackendInternal` and preserving its location.
    ///
    /// # Examples
    ///
    /// ```
    /// use mayon_vulkan_backend::Error;
    /// use mayon_core::CreateBackendError;
    ///
    /// // Obtain an `Error` from the backend (placeholder).
    /// let err: Error = unimplemented!();
    ///
    /// // Convert into a `CreateBackendError`.
    /// let backend_err: CreateBackendError<_> = CreateBackendError::from(err);
    /// ```
    fn from(value: self::VulkanError) -> Self {
        Self::new(BackendInternal(value.kind), value.location)
    }
}
