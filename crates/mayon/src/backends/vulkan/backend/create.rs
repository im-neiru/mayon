use core::{alloc::Allocator, ffi::CStr, mem::MaybeUninit, ptr::NonNull};

use crate::backends::{
    CreateBackend,
    vulkan::{
        Error, VulkanBackend,
        backend::FnTable,
        types::{AllocationCallbacks, ApplicationInfo, InstanceCreateInfo},
    },
};

impl<'s, 'b, A> CreateBackend<'s, A> for VulkanBackend<'b, A>
where
    A: Allocator + 'static,
{
    type Error = Error;
    type Params = VulkanBackendParams<'s>;

    fn create<'a>(allocator: &A, params: Self::Params) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let fns = FnTable::global()?;

        let application_info = ApplicationInfo::new(params);
        let layers = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
        let extensions = [c"VK_KHR_surface".as_ptr()];

        let info = InstanceCreateInfo::new(&application_info, &layers, &extensions);

        let allocation_callbacks = AllocationCallbacks::new(unsafe {
            NonNull::new_unchecked((allocator as *const A).cast_mut())
        });

        let mut instance = MaybeUninit::uninit();

        let instance = unsafe {
            (fns.fn_create_instance)(&info, allocation_callbacks.alloc_ref(), &mut instance)
                .into_result("vkCreateInstance", || instance.assume_init())?
        };

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
}

impl Default for VulkanBackendParams<'_> {
    fn default() -> Self {
        let v0_1 = VulkanVersion::new(0, 1, 0);

        Self {
            application_name: None,
            application_version: v0_1,
            engine_name: None,
            engine_version: v0_1,
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
