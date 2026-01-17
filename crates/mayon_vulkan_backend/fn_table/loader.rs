use libloading::{Error, Library};

#[cfg(target_os = "windows")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(super) unsafe fn vulkan_lib() -> Result<Library, Error> {
    Library::new("vulkan-1.dll")
}

#[cfg(target_os = "linux")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(super) unsafe fn vulkan_lib() -> Result<Library, Error> {
    let result = Library::new("libvulkan.so.1");

    if result.is_ok() {
        return result;
    }

    Library::new("libvulkan.so")
}

#[cfg(target_os = "macos")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(super) unsafe fn vulkan_lib() -> Result<Library, Error> {
    let mut result = Library::new("libvulkan.1.dylib");

    if result.is_ok() {
        return result;
    }

    result = Library::new("libvulkan.dylib");

    if result.is_ok() {
        return result;
    }

    Library::new("MoltenVK.dylib")
}

#[cfg(target_os = "ios")]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub(super) unsafe fn vulkan_lib() -> Result<Library, Error> {
    let result = Library::new("libMoltenVK.dylib");

    if result.is_ok() {
        return result;
    }

    Library::new("vulkan.framework/vulkan")
}
