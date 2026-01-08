#pragma once
#include <stdint.h>

typedef struct VulkanVersion {
    uint32_t major;
    uint32_t minor;
    uint32_t patch;
} VulkanVersion;

typedef struct VulkanBackendParams {
    const char* application_name;
    VulkanVersion application_version;
    const char* engine_name;
    VulkanVersion engine_version;
} VulkanBackendParams;

int mayon_new_instance_on_vulkan(
    const VulkanBackendParams* params,
    Instance* out_instance
);
