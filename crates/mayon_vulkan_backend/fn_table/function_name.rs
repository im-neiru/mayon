use strum::{Display, IntoStaticStr};

#[derive(Copy, Clone, Debug, IntoStaticStr, Display, PartialEq, Eq)]
pub enum VulkanFunctionName {
    #[strum(serialize = "vkCreateInstance")]
    CreateInstance,
    #[strum(serialize = "vkDestroyInstance")]
    DestroyInstance,
    #[strum(serialize = "vkCreateWin32SurfaceKHR")]
    CreateWin32Surface,
    #[strum(serialize = "vkCreateWaylandSurfaceKHR")]
    CreateWaylandSurface,
    #[strum(serialize = "vkCreateXcbSurfaceKHR")]
    CreateXcbSurface,
    #[strum(serialize = "vkCreateXlibSurfaceKHR")]
    CreateXlibSurface,
    #[strum(serialize = "vkDestroySurfaceKHR")]
    DestroySurface,
    #[strum(serialize = "vkEnumerateInstanceLayerProperties")]
    EnumerateInstanceLayerProperties,
    #[strum(serialize = "vkEnumeratePhysicalDevices")]
    EnumeratePhysicalDevices,
    #[strum(serialize = "vkGetPhysicalDeviceQueueFamilyProperties")]
    GetPhysicalDeviceQueueFamilyProperties,
}

impl AsRef<str> for VulkanFunctionName {
    #[inline]
    fn as_ref(&self) -> &str {
        self.into()
    }
}
