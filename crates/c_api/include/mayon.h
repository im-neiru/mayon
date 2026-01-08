#ifndef MAYON_API_H
#define MAYON_API_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct VulkanVersion {
  uint32_t major;
  uint32_t minor;
  uint32_t patch;
} VulkanVersion;

typedef struct VulkanBackendParams {
  const char *application_name;
  struct VulkanVersion application_version;
  const char *engine_name;
  struct VulkanVersion engine_version;
} VulkanBackendParams;

typedef struct Instance {
  uintptr_t _0;
} Instance;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int32_t mayon_new_instance_on_vulkan(const struct VulkanBackendParams *param,
                                     struct Instance *out_instance);

void mayon_drop_instance(struct Instance *instance);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* MAYON_API_H */
