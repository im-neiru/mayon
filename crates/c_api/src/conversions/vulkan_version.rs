use crate::rs;

impl From<crate::VulkanVersion> for rs::VulkanVersion {
    #[inline(always)]
    fn from(value: crate::VulkanVersion) -> Self {
        let crate::VulkanVersion {
            major,
            minor,
            patch,
        } = value;

        Self {
            major,
            minor,
            patch,
        }
    }
}
