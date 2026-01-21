use core::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use raw_window_handle::HasDisplayHandle;

use allocator::Allocator;
use mayon_core::{
    BaseError, CreateBackend, CreateBackendError, TargetPlatform, UnsupportedPlatformError, info,
    logger::{Logger, Target as LogTarget},
};
use utils::{BufferOverflowError, InlineVec};

use crate::{
    VulkanBackend, VulkanError,
    backend::FnTable,
    types::{AllocationCallbacks, ApplicationInfo, ExtensionName, InstanceCreateInfo},
};

impl<'s, L, A> CreateBackend<'s, A, L> for VulkanBackend<'static, L, A>
where
    L: Logger,
    A: Allocator + 'static,
{
    type Error = VulkanError;
    type Params = VulkanBackendParams<'s>;

    /// Creates a new Vulkan backend instance configured by `params`.
    ///
    /// The function constructs a Vulkan instance using the provided application/engine
    /// metadata and optional platform surface extensions from `params.target_platform`,
    /// installs allocation callbacks that use `allocator`, and logs successful creation
    /// to `logger`. It returns the constructed `VulkanBackend` on success.
    ///
    /// Panics if the internal extension-name buffer capacity is exceeded.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` with the created `VulkanBackend` on success, `Err(CreateBackendError<...>)`
    /// if the global Vulkan function table cannot be obtained or instance creation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::backends::vulkan::{VulkanBackend, VulkanBackendParams};
    /// # use crate::alloc::DefaultAllocator;
    /// # use crate::logging::StdLogger;
    /// let allocator = DefaultAllocator::default();
    /// let mut logger = StdLogger::new();
    /// let params = VulkanBackendParams::default();
    /// let backend = VulkanBackend::create(&allocator, &mut logger, params).unwrap();
    /// ```
    fn create<'a>(
        allocator: &A,
        logger: &mut L,
        params: Self::Params,
    ) -> Result<Self, CreateBackendError<<Self::Error as BaseError>::ErrorKind>>
    where
        Self: Sized,
    {
        let fns = FnTable::global()?;

        let application_info = ApplicationInfo::new(params);

        #[cfg(debug_assertions)]
        let layers = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
        #[cfg(not(debug_assertions))]
        let layers = [];

        let mut extensions = InlineVec::<ExtensionName, 12>::new();

        if let Some(target_platform) = params.target_platform {
            append_extension_names(&target_platform, &mut extensions)
                .expect("Vulkan extension name buffer overflow");
        }

        let info = InstanceCreateInfo::new(&application_info, &layers, extensions.as_slice());

        let allocation_callbacks = AllocationCallbacks::new(unsafe {
            NonNull::new_unchecked((allocator as *const A).cast_mut())
        });

        let instance = unsafe { fns.create_instance(&info, allocation_callbacks.alloc_ref()) }?;

        info!(logger, LogTarget::Backend, "Vulkan instance created");

        Ok(Self {
            instance,
            alloc: allocation_callbacks,
            _marker: PhantomData,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VulkanBackendParams<'s> {
    pub application_name: Option<&'s CStr>,
    pub application_version: VulkanVersion,
    pub engine_name: Option<&'s CStr>,
    pub engine_version: VulkanVersion,
    pub target_platform: Option<TargetPlatform>,
}

impl Default for VulkanBackendParams<'_> {
    /// Creates a default `VulkanBackendParams` with application and engine versions set to 0.1.0 and no names or target platform.
    ///
    /// # Examples
    ///
    /// ```
    /// let params = VulkanBackendParams::default();
    /// assert!(params.application_name.is_none());
    /// assert_eq!(params.application_version, VulkanVersion::new(0, 1, 0));
    /// assert!(params.engine_name.is_none());
    /// assert_eq!(params.engine_version, VulkanVersion::new(0, 1, 0));
    /// assert!(params.target_platform.is_none());
    /// ```
    fn default() -> Self {
        let v0_1 = VulkanVersion::new(0, 1, 0);

        Self {
            application_name: None,
            application_version: v0_1,
            engine_name: None,
            engine_version: v0_1,
            target_platform: None,
        }
    }
}

impl<'s> VulkanBackendParams<'s> {
    #[inline]
    pub fn with_application_name(mut self, application_name: impl Into<&'s CStr>) -> Self {
        self.application_name = Some(application_name.into());
        self
    }

    #[inline]
    pub fn with_application_version(
        mut self,
        application_version: impl Into<VulkanVersion>,
    ) -> Self {
        self.application_version = application_version.into();
        self
    }

    #[inline]
    pub fn with_engine_name(mut self, engine_name: impl Into<&'s CStr>) -> Self {
        self.engine_name = Some(engine_name.into());
        self
    }

    /// Sets the engine version in the parameters and returns the updated params.
    ///
    /// # Parameters
    ///
    /// - `engine_version`: The engine version to store in the params.
    ///
    /// # Returns
    ///
    /// The updated `VulkanBackendParams` with `engine_version` set to the provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// let params = crate::backends::vulkan::VulkanBackendParams::default()
    ///     .with_engine_version((1u32, 2u32, 3u32));
    /// assert_eq!(params.engine_version, crate::backends::vulkan::VulkanVersion::new(1, 2, 3));
    /// ```
    #[inline]
    pub fn with_engine_version(mut self, engine_version: impl Into<VulkanVersion>) -> Self {
        self.engine_version = engine_version.into();
        self
    }

    /// Sets the backend's target platform from a raw-window-handle display and returns the updated params.
    ///
    /// If `display` is `Some`, converts it to a `TargetPlatform` using `TargetPlatform::from_raw_display_handle`
    /// with `with_headless` and stores it in `target_platform`. If `display` is `None`, clears `target_platform`.
    ///
    /// # Errors
    ///
    /// Returns `UnsupportedPlatformError` if the provided display handle is not supported on the current platform.
    ///
    /// # Examples
    ///
    /// ```
    /// let params = VulkanBackendParams::default();
    /// let params = params.with_target_from_rwh::<raw_window_handle::RawDisplayHandle>(None, false).unwrap();
    /// assert!(params.target_platform.is_none());
    /// ```
    pub fn with_target_from_rwh(
        mut self,
        display: Option<impl HasDisplayHandle>,
        with_headless: bool,
    ) -> Result<Self, UnsupportedPlatformError> {
        if let Some(display) = display.as_ref() {
            let Ok(display) = display.display_handle() else {
                return Err(UnsupportedPlatformError);
            };

            let target = TargetPlatform::from_raw_display_handle(display.as_raw(), with_headless)?;

            self.target_platform = Some(target);
        } else {
            self.target_platform = None;
        }

        Ok(self)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VulkanVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl VulkanVersion {
    /// Creates a VulkanVersion from its major, minor, and patch components.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = VulkanVersion::new(1, 2, 3);
    /// assert_eq!(v.major, 1);
    /// assert_eq!(v.minor, 2);
    /// assert_eq!(v.patch, 3);
    /// ```
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Encodes the version components into a single 32-bit Vulkan-style version value.
    ///
    /// The bits are laid out as `(major << 22) | (minor << 12) | patch`, matching Vulkan's packed version format.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = VulkanVersion::new(1, 2, 3);
    /// assert_eq!(v.raw(), (1u32 << 22) | (2u32 << 12) | 3u32);
    /// ```
    pub(crate) const fn raw(&self) -> u32 {
        (self.major << 22) | (self.minor << 12) | self.patch
    }
}

impl From<(u32, u32, u32)> for VulkanVersion {
    #[inline]
    fn from((major, minor, patch): (u32, u32, u32)) -> Self {
        Self::new(major, minor, patch)
    }
}

impl From<(u32, u32)> for VulkanVersion {
    /// Creates a VulkanVersion from a `(major, minor)` pair with `patch` set to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = VulkanVersion::from((1u32, 2u32));
    /// assert_eq!(v, VulkanVersion::new(1, 2, 0));
    /// ```
    fn from((major, minor): (u32, u32)) -> Self {
        Self::new(major, minor, 0)
    }
}

/// Appends the Vulkan surface extension names required for the specified target platforms.
///
/// Given a set of target platforms, pushes the corresponding `ExtensionName` entries into
/// `buffer` for each matching platform (or the generic `SURFACE` entry for `HEADLESS`).
///
/// # Parameters
///
/// - `targets`: Target platforms to inspect for required surface extensions.
/// - `buffer`: Inline vector to receive the extension names; must have sufficient capacity.
///
/// # Returns
///
/// `Ok(())` on success, `Err(BufferOverflowError)` if `buffer` does not have enough capacity.
///
/// # Examples
///
/// ```
/// use inline_vec::InlineVec;
/// // types `TargetPlatform` and `ExtensionName` are assumed to be in scope for the example
/// let targets = TargetPlatform::WIN32;
/// let mut buf = InlineVec::<ExtensionName, 4>::new();
/// let res = append_extension_names(&targets, &mut buf);
/// assert!(res.is_ok());
/// ```
#[inline(always)]
fn append_extension_names<const CAPACITY: usize>(
    targets: &TargetPlatform,
    buffer: &mut InlineVec<ExtensionName, CAPACITY>,
) -> Result<(), BufferOverflowError> {
    if !targets.is_empty() {
        buffer.push(ExtensionName::SURFACE)?;
    }

    if targets.contains(TargetPlatform::WAYLAND) {
        buffer.push(ExtensionName::WAYLAND_SURFACE)?;
    }

    if targets.contains(TargetPlatform::XCB) {
        buffer.push(ExtensionName::XCB_SURFACE)?;
    }

    if targets.contains(TargetPlatform::XLIB) {
        buffer.push(ExtensionName::XLIB_SURFACE)?;
    }

    if targets.contains(TargetPlatform::WIN32) {
        buffer.push(ExtensionName::WIN32_SURFACE)?;
    }

    if targets.contains(TargetPlatform::ANDROID) {
        buffer.push(ExtensionName::ANDROID_SURFACE)?;
    }

    if targets.contains(TargetPlatform::METAL) {
        buffer.push(ExtensionName::MACOS_SURFACE)?;
    }

    Ok(())
}
