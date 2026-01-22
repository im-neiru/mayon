#[repr(C)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: [u32; 3],
}

bitflags::bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueueFlags: u32 {
        const GRAPHICS = 0x00000001;
        const COMPUTE = 0x00000002;
        const TRANSFER = 0x00000004;
        const SPARSE_BINDING = 0x00000008;
    }
}
