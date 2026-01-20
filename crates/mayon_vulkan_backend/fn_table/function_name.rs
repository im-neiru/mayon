use strum::{Display, IntoStaticStr};

#[derive(Copy, Clone, Debug, IntoStaticStr, Display, PartialEq, Eq)]
pub enum VulkanFunctionName {
    #[strum(serialize = "vkCreateInstance")]
    CreateInstance,
    #[strum(serialize = "vkDestroyInstance")]
    DestroyInstance,
    #[strum(serialize = "vkCreateWin32SurfaceKHR")]
    CreateWin32Surface,
}

impl AsRef<str> for VulkanFunctionName {
    #[inline]
    fn as_ref(&self) -> &str {
        self.into()
    }
}
