use core::ffi::c_char;

const MAX_EXTENSION_NAME_SIZE: usize = 256;
const MAX_DESCRIPTION_SIZE: usize = 256;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct LayerProperties {
    pub(super) layer_name: [c_char; MAX_EXTENSION_NAME_SIZE],
    pub(super) spec_version: u32,
    pub(super) implementation_version: u32,
    pub(super) description: [c_char; MAX_DESCRIPTION_SIZE],
}

impl LayerProperties {
    #[inline]
    pub const fn zeroized<const N: usize>() -> [Self; N] {
        [Self {
            layer_name: [0; MAX_EXTENSION_NAME_SIZE],
            spec_version: 0,
            implementation_version: 0,
            description: [0; MAX_DESCRIPTION_SIZE],
        }; N]
    }
}
