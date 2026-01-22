mod function_name;
mod loader;

pub use function_name::VulkanFunctionName;

use core::mem::MaybeUninit;

use libloading::Library;
use once_cell::sync::OnceCell;

use VulkanFunctionName::*;

use crate::{VulkanErrorKind, types::*};

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

    fn_enumerate_instance_layer_properties: unsafe extern "system" fn(
        property_count: *mut u32,
        properties: *mut LayerProperties,
    ) -> VkResult,

    fn_enumerate_physical_devices: unsafe extern "system" fn(
        instance: Instance,
        physical_device_count: *mut u32,
        physical_devices: *mut PhysicalDevice,
    ) -> VkResult,

    fn_get_physical_device_queue_family_properties: unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_property_count: *mut u32,
        queue_family_properties: *mut QueueFamilyProperties,
    ) -> VkResult,

    fn_get_physical_device_surface_support: unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: Surface,
        supported: *mut Bool32,
    ) -> VkResult,
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
                fn_enumerate_instance_layer_properties: unsafe {
                    *library
                        .get(EnumerateInstanceLayerProperties.as_ref())
                        .map_err(|_| VulkanErrorKind::FunctionLoadFailed {
                            name: EnumerateInstanceLayerProperties,
                        })?
                },
                fn_enumerate_physical_devices: unsafe {
                    *library
                        .get(EnumeratePhysicalDevices.as_ref())
                        .map_err(|_| VulkanErrorKind::FunctionLoadFailed {
                            name: EnumeratePhysicalDevices,
                        })?
                },
                fn_get_physical_device_queue_family_properties: unsafe {
                    *library
                        .get(GetPhysicalDeviceQueueFamilyProperties.as_ref())
                        .map_err(|_| VulkanErrorKind::FunctionLoadFailed {
                            name: GetPhysicalDeviceQueueFamilyProperties,
                        })?
                },
                fn_get_physical_device_surface_support: unsafe {
                    *library
                        .get(GetPhysicalDeviceSurfaceSupport.as_ref())
                        .map_err(|_| VulkanErrorKind::FunctionLoadFailed {
                            name: GetPhysicalDeviceSurfaceSupport,
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

    #[inline]
    pub(crate) unsafe fn enumerate_instance_layer_properties(
        &self,
        property_count: &mut u32,
        properties: *mut LayerProperties,
    ) -> super::Result<()> {
        unsafe { (self.fn_enumerate_instance_layer_properties)(property_count, properties) }
            .into_result(EnumerateInstanceLayerProperties, || ())
    }

    #[inline]
    pub(crate) unsafe fn enumerate_physical_devices(
        &self,
        instance: Instance,
        physical_device_count: &mut u32,
        physical_devices: *mut PhysicalDevice,
    ) -> super::Result<()> {
        unsafe {
            (self.fn_enumerate_physical_devices)(instance, physical_device_count, physical_devices)
        }
        .into_result(EnumeratePhysicalDevices, || ())
    }

    #[inline]
    pub(crate) unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        queue_family_property_count: &mut u32,
        queue_family_properties: *mut QueueFamilyProperties,
    ) -> super::Result<()> {
        unsafe {
            (self.fn_get_physical_device_queue_family_properties)(
                physical_device,
                queue_family_property_count,
                queue_family_properties,
            )
        }
        .into_result(GetPhysicalDeviceQueueFamilyProperties, || ())
    }

    #[inline]
    pub(crate) unsafe fn get_physical_device_surface_support(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: Surface,
    ) -> super::Result<bool> {
        let mut supported = Bool32::False;

        unsafe {
            (self.fn_get_physical_device_surface_support)(
                physical_device,
                queue_family_index,
                surface,
                &mut supported,
            )
        }
        .into_result(GetPhysicalDeviceSurfaceSupport, || supported.into())
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
