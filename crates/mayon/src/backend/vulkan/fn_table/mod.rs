use std::sync::OnceLock;

mod loader;

use libloading::Library;

use crate::backend::vulkan::ErrorKind;

pub struct FnTable {
    library: Option<Library>,
}

static FN_TABLE: OnceLock<FnTable> = OnceLock::new();

impl FnTable {
    pub(in crate::backend) fn global() -> super::Result<&'static Self> {
        FN_TABLE.get_or_try_init(Self::new)
    }

    fn new() -> super::Result<Self> {
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
