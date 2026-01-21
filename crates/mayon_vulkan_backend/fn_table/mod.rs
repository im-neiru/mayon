mod function_name;
mod loader;

pub use function_name::VulkanFunctionName;

use core::mem::MaybeUninit;

use libloading::Library;
use once_cell::sync::OnceCell;

use VulkanFunctionName::*;

use crate::{
    VulkanErrorKind,
    types::{
        AllocationCallbacksRef, Instance, InstanceCreateInfo, Surface, VkResult,
        WaylandSurfaceCreateInfo, Win32SurfaceCreateInfo, XcbSurfaceCreateInfo,
        XlibSurfaceCreateInfo,
    },
};

pub struct FnTable {
    library: Option<Library>,

    fn_create_instance: unsafe extern "system" fn(
        create_info: *const InstanceCreateInfo,
        allocator: AllocationCallbacksRef,
        instance: *mut Instance,
    ) -> VkResult,

    fn_destroy_instance:
        unsafe extern "system" fn(instance: Instance, allocator: AllocationCallbacksRef),

    fn_create_win32_surface: Option<
        unsafe extern "system" fn(
            instance: Instance,
            create_info: *const Win32SurfaceCreateInfo,
            allocator: AllocationCallbacksRef,
            surface: *mut Surface,
        ) -> VkResult,
    >,

    fn_create_wayland_surface: Option<
        unsafe extern "system" fn(
            instance: Instance,
            create_info: *const WaylandSurfaceCreateInfo,
            allocator: AllocationCallbacksRef,
            surface: *mut Surface,
        ) -> VkResult,
    >,
    fn_create_xcb_surface: Option<
        unsafe extern "system" fn(
            instance: Instance,
            create_info: *const XcbSurfaceCreateInfo,
            allocator: AllocationCallbacksRef,
            surface: *mut Surface,
        ) -> VkResult,
    >,
    fn_create_xlib_surface: Option<
        unsafe extern "system" fn(
            instance: Instance,
            create_info: *const XlibSurfaceCreateInfo,
            allocator: AllocationCallbacksRef,
            surface: *mut Surface,
        ) -> VkResult,
    >,

    fn_destroy_surface: unsafe extern "system" fn(
        instance: Instance,
        surface: Surface,
        allocator: AllocationCallbacksRef,
    ),
}

static FN_TABLE: OnceCell<FnTable> = OnceCell::new();

impl FnTable {
    /// Returns the cached global function table, initializing and caching it on first use.
    ///
    /// On success, yields a `'static` reference to the initialized `FnTable`. Returns an error if the Vulkan library cannot be loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// // Access the global function table (may initialize the table on first call).
    /// let table = crate::backends::vulkan::FnTable::global().unwrap();
    /// // Use `table` to call loaded Vulkan functions, e.g. `table.fn_create_instance`.
    /// ```
    pub(crate) fn global() -> super::Result<&'static Self> {
        FN_TABLE.get_or_try_init(Self::new)
    }

    fn new() -> super::Result<Self> {
        match unsafe { loader::vulkan_lib() } {
            Ok(library) => Ok(Self {
                fn_create_instance: unsafe {
                    *library.get(CreateInstance.as_ref()).map_err(|_| {
                        VulkanErrorKind::FunctionLoadFailed {
                            name: CreateInstance,
                        }
                    })?
                },
                fn_destroy_instance: unsafe {
                    *library.get(DestroyInstance.as_ref()).map_err(|_| {
                        VulkanErrorKind::FunctionLoadFailed {
                            name: DestroyInstance,
                        }
                    })?
                },
                fn_create_win32_surface: unsafe {
                    library
                        .get(CreateWin32Surface.as_ref())
                        .map(|ptr| *ptr)
                        .ok()
                },
                fn_create_wayland_surface: unsafe {
                    library
                        .get(CreateWaylandSurface.as_ref())
                        .map(|ptr| *ptr)
                        .ok()
                },
                fn_create_xcb_surface: unsafe {
                    library.get(CreateXcbSurface.as_ref()).map(|ptr| *ptr).ok()
                },
                fn_create_xlib_surface: unsafe {
                    library.get(CreateXlibSurface.as_ref()).map(|ptr| *ptr).ok()
                },
                fn_destroy_surface: unsafe {
                    *library.get(DestroySurface.as_ref()).map_err(|_| {
                        VulkanErrorKind::FunctionLoadFailed {
                            name: DestroySurface,
                        }
                    })?
                },
                library: Some(library),
            }),
            Err(_) => VulkanErrorKind::LibraryLoad.into_result(),
        }
    }
}

impl FnTable {
    #[inline]
    pub(crate) unsafe fn create_instance(
        &self,
        create_info: &InstanceCreateInfo,
        allocator: AllocationCallbacksRef,
    ) -> super::Result<Instance> {
        let mut instance = MaybeUninit::<Instance>::uninit();

        unsafe { (self.fn_create_instance)(create_info, allocator, instance.as_mut_ptr()) }
            .into_result(CreateInstance, || unsafe { instance.assume_init() })
    }

    #[inline]
    pub(crate) unsafe fn destroy_instance(
        &self,
        instance: Instance,
        allocator: AllocationCallbacksRef,
    ) {
        unsafe { (self.fn_destroy_instance)(instance, allocator) }
    }

    #[inline]
    pub(crate) unsafe fn create_win32_surface(
        &self,
        instance: Instance,
        create_info: &Win32SurfaceCreateInfo,
        allocator: AllocationCallbacksRef,
    ) -> super::Result<Surface> {
        let Some(fn_create_win32_surface) = self.fn_create_win32_surface else {
            return VulkanErrorKind::FunctionLoadFailed {
                name: CreateWin32Surface,
            }
            .into_result();
        };

        let mut surface = MaybeUninit::<Surface>::uninit();

        unsafe { (fn_create_win32_surface)(instance, create_info, allocator, surface.as_mut_ptr()) }
            .into_result(CreateWin32Surface, || unsafe { surface.assume_init() })
    }

    #[inline]
    pub(crate) unsafe fn create_wayland_surface(
        &self,
        instance: Instance,
        create_info: &WaylandSurfaceCreateInfo,
        allocator: AllocationCallbacksRef,
    ) -> super::Result<Surface> {
        let Some(fn_create_wayland_surface) = self.fn_create_wayland_surface else {
            return VulkanErrorKind::FunctionLoadFailed {
                name: CreateWaylandSurface,
            }
            .into_result();
        };

        let mut surface = MaybeUninit::<Surface>::uninit();

        unsafe {
            (fn_create_wayland_surface)(instance, create_info, allocator, surface.as_mut_ptr())
        }
        .into_result(CreateWaylandSurface, || unsafe { surface.assume_init() })
    }

    #[inline]
    pub(crate) unsafe fn create_xcb_surface(
        &self,
        instance: Instance,
        create_info: &XcbSurfaceCreateInfo,
        allocator: AllocationCallbacksRef,
    ) -> super::Result<Surface> {
        let Some(fn_create_xcb_surface) = self.fn_create_xcb_surface else {
            return VulkanErrorKind::FunctionLoadFailed {
                name: CreateXcbSurface,
            }
            .into_result();
        };

        let mut surface = MaybeUninit::<Surface>::uninit();

        unsafe { (fn_create_xcb_surface)(instance, create_info, allocator, surface.as_mut_ptr()) }
            .into_result(CreateXcbSurface, || unsafe { surface.assume_init() })
    }

    #[inline]
    pub(crate) unsafe fn create_xlib_surface(
        &self,
        instance: Instance,
        create_info: &XlibSurfaceCreateInfo,
        allocator: AllocationCallbacksRef,
    ) -> super::Result<Surface> {
        let Some(fn_create_xlib_surface) = self.fn_create_xlib_surface else {
            return VulkanErrorKind::FunctionLoadFailed {
                name: CreateXlibSurface,
            }
            .into_result();
        };

        let mut surface = MaybeUninit::<Surface>::uninit();

        unsafe { (fn_create_xlib_surface)(instance, create_info, allocator, surface.as_mut_ptr()) }
            .into_result(CreateXlibSurface, || unsafe { surface.assume_init() })
    }

    #[inline]
    pub(crate) unsafe fn destroy_surface(
        &self,
        instance: Instance,
        surface: Surface,
        allocator: AllocationCallbacksRef,
    ) {
        unsafe { (self.fn_destroy_surface)(instance, surface, allocator) }
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
