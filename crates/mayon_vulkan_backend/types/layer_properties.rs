use core::ffi::c_char;

const MAX_EXTENSION_NAME_SIZE: usize = 256;
const MAX_DESCRIPTION_SIZE: usize = 256;

pub(crate) struct LayerProperties {
    pub(super) layer_name: [c_char; MAX_EXTENSION_NAME_SIZE],
    pub(super) spec_version: u32,
    pub(super) implementation_version: u32,
    pub(super) description: [c_char; MAX_DESCRIPTION_SIZE],
}
