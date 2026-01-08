mod loader;

use libloading::Library;

pub struct FnTable {
    library: Option<Library>,
}

impl FnTable {
    pub(crate) fn new() -> Self {
        let library = unsafe { loader::vulkan_lib().unwrap() };

        Self {
            library: Some(library),
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
