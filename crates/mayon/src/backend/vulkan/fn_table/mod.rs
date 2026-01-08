mod loader;

use libloading::Library;

use crate::backend::vulkan::ErrorKind;

pub struct FnTable {
    library: Option<Library>,
}

impl FnTable {
    pub(crate) fn new() -> super::Result<Self> {
        match unsafe { loader::vulkan_lib() } {
            Ok(library) => Ok(Self {
                library: Some(library),
            }),
            Err(err) => {
                #[cfg(debug_assertions)]
                eprintln!("Vulkan load error: {err}");

                ErrorKind::VulkanLoad.into_result()
            }
        }
    }
}

impl Drop for FnTable {
    #[inline]
    fn drop(&mut self) {
        if let Some(library) = self.library.take() {
            library.close().unwrap();
        }
    }
}
