use crate::rs;

impl From<crate::MynVkVersion> for rs::VulkanVersion {
    #[inline(always)]
    fn from(value: crate::MynVkVersion) -> Self {
        let crate::MynVkVersion {
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
