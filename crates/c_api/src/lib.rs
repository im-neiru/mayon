mod conversions;

use core::ffi::c_char;

mod rs {
    pub(super) use mayon::{
        Instance,
        backends::vulkan::{VulkanBackend, VulkanBackendParams, VulkanVersion},
    };
}

#[repr(C)]
pub struct VulkanBackendParams {
    pub application_name: *const c_char,
    pub application_version: VulkanVersion,
    pub engine_name: *const c_char,
    pub engine_version: VulkanVersion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VulkanVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[repr(C)]
pub struct Instance(usize);

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn mayon_new_instance_on_vulkan(
    param: *const VulkanBackendParams,
    out_instance: *mut Instance,
) -> i32 {
    if param.is_null() || out_instance.is_null() {
        return -1;
    }

    let param = unsafe { &*param };

    let rust_params = rs::VulkanBackendParams {
        application_name: conversions::ptr_to_op_cstr(param.application_name),
        application_version: param.application_version.into(),
        engine_name: conversions::ptr_to_op_cstr(param.engine_name),
        engine_version: param.engine_version.into(),
    };

    match rs::Instance::new::<'static, rs::VulkanBackend>(rust_params) {
        Ok(instance) => {
            unsafe { out_instance.write(instance.into()) };
            0
        }
        Err(_) => -1,
    }
}
