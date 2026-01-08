#ifndef MAYON_API_H
#define MAYON_API_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Vulkan version structure.
 */
typedef struct VulkanVersion {
  uint32_t major;
  uint32_t minor;
  uint32_t patch;
} VulkanVersion;

/**
 * Vulkan backend initialization parameters.
 *
 * All pointer fields are borrowed for the duration of the call.
 */
typedef struct VulkanBackendParams {
  /**
   * Optional null-terminated UTF-8 application name.
   */
  const char *application_name;
  /**
   * Application version.
   */
  struct VulkanVersion application_version;
  /**
   * Optional null-terminated UTF-8 engine name.
   */
  const char *engine_name;
  /**
   * Engine version.
   */
  struct VulkanVersion engine_version;
} VulkanBackendParams;

/**
 * Opaque Mayon instance handle.
 *
 * Instances are reference-counted internally.
 */
typedef struct Instance {
  uintptr_t _0;
} Instance;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Creates a new Mayon instance using the Vulkan backend.
 *
 * Returns `0` on success and writes a valid [`Instance`] to `out_instance`.
 * Returns a non-zero value on failure.
 *
 * # Safety
 *
 * - `param` must point to a valid [`VulkanBackendParams`].
 * - `out_instance` must point to writable, properly aligned storage for
 *   an [`Instance`].
 * - If non-null, string pointers in `param` must be valid, null-terminated
 *   UTF-8 strings for the duration of the call.
 *
 * On failure, `out_instance` is not written.
 */
int32_t mayon_new_instance_on_vulkan(const struct VulkanBackendParams *param,
                                     struct Instance *out_instance);

/**
 * Releases a Mayon instance.
 *
 * Passing a null pointer has no effect.
 *
 * # Safety
 *
 * - `instance` must be a pointer obtained from this API or null.
 * - The instance must not be released more times than it was created.
 *
 * Instances are internally reference-counted. Releasing the same instance
 * multiple times may cause unintended deallocation once the reference count
 * reaches zero.
 */
void mayon_drop_instance(struct Instance *instance);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* MAYON_API_H */
