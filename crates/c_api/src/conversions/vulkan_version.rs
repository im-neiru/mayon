impl From<crate::MynVkVersion> for mayon::backends::vulkan::VulkanVersion {
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
