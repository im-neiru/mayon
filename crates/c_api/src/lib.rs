use core::ffi::{CStr, c_char};

use mayon::{
    Instance,
    backends::vulkan::{VulkanBackend, VulkanBackendParams, VulkanVersion},
};

#[repr(C)]
pub struct CVulkanBackendParams {
    pub application_name: *const c_char,
    pub application_version: VulkanVersion,
    pub engine_name: *const c_char,
    pub engine_version: VulkanVersion,
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn mayon_new_instance_on_vulkan(
    param: *const CVulkanBackendParams,
    out_instance: *mut Instance,
) -> i32 {
    if param.is_null() || out_instance.is_null() {
        return -1;
    }

    let param = unsafe { &*param };

    let to_cstr = |ptr: *const c_char| {
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    };

    let rust_params = VulkanBackendParams {
        application_name: to_cstr(param.application_name),
        application_version: param.application_version,
        engine_name: to_cstr(param.engine_name),
        engine_version: param.engine_version,
    };

    match Instance::new::<'static, VulkanBackend>(rust_params) {
        Ok(instance) => {
            unsafe { out_instance.write(instance) };
            0
        }
        Err(_) => 1,
    }
}
