use core::{alloc::Allocator, ffi::CStr, mem::MaybeUninit, ptr::NonNull};

use utils::{BufferOverflowError, InlineVec};

use crate::{
    BaseError,
    backends::{
        CreateBackend, CreateError, TargetPlatform, UnsupportedPlatformError,
        vulkan::{
            Error, VulkanBackend,
            backend::FnTable,
            types::{AllocationCallbacks, ApplicationInfo, ExtensionName, InstanceCreateInfo},
        },
    },
    logger::{Logger, Target as LogTarget},
};

impl<'s, 'b, A, L> CreateBackend<'s, A, L> for VulkanBackend<'b, A>
where
    A: Allocator + 'static,
    L: Logger + 'static,
{
    type Error = Error;
    type Params = VulkanBackendParams<'s>;

    fn create<'a>(
        allocator: &A,
        logger: &mut L,
        params: Self::Params,
    ) -> Result<Self, CreateError<<Self::Error as BaseError>::ErrorKind>>
    where
        Self: Sized,
    {
        let fns = FnTable::global()?;

        let application_info = ApplicationInfo::new(params);
        let layers = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
        let mut extensions = InlineVec::<ExtensionName, 12>::new();

        if let Some(target_platform) = params.target_platform {
            target_platform
                .append_extension_names(&mut extensions)
                .expect("Vulkan extension name buffer overflow");
        }

        let info = InstanceCreateInfo::new(&application_info, &layers, extensions.as_slice());

        let allocation_callbacks = AllocationCallbacks::new(unsafe {
            NonNull::new_unchecked((allocator as *const A).cast_mut())
        });

        let mut instance = MaybeUninit::uninit();

        let instance = unsafe {
            (fns.fn_create_instance)(&info, allocation_callbacks.alloc_ref(), &mut instance)
                .into_result("vkCreateInstance", || instance.assume_init())?
        };

        crate::info!(logger, LogTarget::Backend, "Vulkan instance created");

        Ok(Self {
            instance,
            alloc: allocation_callbacks,
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

    #[inline]
    pub fn with_engine_version(mut self, engine_version: impl Into<VulkanVersion>) -> Self {
        self.engine_version = engine_version.into();
        self
    }

    #[inline]
    pub fn with_target_from_rwh(
        mut self,
        display: Option<impl Into<raw_window_handle::RawDisplayHandle>>,
        with_headless: bool,
    ) -> Result<Self, UnsupportedPlatformError> {
        if let Some(display) = display {
            let target = TargetPlatform::from_raw_display_handle(display.into(), with_headless)?;

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
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub(in crate::backends::vulkan) const fn raw(&self) -> u32 {
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
    #[inline]
    fn from((major, minor): (u32, u32)) -> Self {
        Self::new(major, minor, 0)
    }
}

impl TargetPlatform {
    #[inline]
    fn append_extension_names<const CAPACITY: usize>(
        &self,
        buffer: &mut InlineVec<ExtensionName, CAPACITY>,
    ) -> Result<(), BufferOverflowError> {
        if *self == Self::HEADLESS {
            buffer.push(ExtensionName::SURFACE)?;
        }

        if self.contains(Self::WAYLAND) {
            buffer.push(ExtensionName::WAYLAND_SURFACE)?;
        }

        if self.contains(Self::XCB) {
            buffer.push(ExtensionName::XCB_SURFACE)?;
        }

        if self.contains(Self::XLIB) {
            buffer.push(ExtensionName::XLIB_SURFACE)?;
        }

        if self.contains(Self::WIN32) {
            buffer.push(ExtensionName::WIN32_SURFACE)?;
        }

        if self.contains(Self::ANDROID) {
            buffer.push(ExtensionName::ANDROID_SURFACE)?;
        }

        if self.contains(Self::METAL) {
            buffer.push(ExtensionName::MACOS_SURFACE)?;
        }

        Ok(())
    }
}
