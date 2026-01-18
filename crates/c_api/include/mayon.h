#ifndef MAYON_API_H
#define MAYON_API_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Numeric result codes returned by Mayon C API functions.
 *
 * The value layout is implementation-defined but stable.
 * Applications should compare against the named constants.
 */
enum MynFallibleResult
#ifdef __cplusplus
  : uint16_t
#endif // __cplusplus
 {
  /**
   * @brief The operation succeeded.
   */
  MAYON_RESULT_OK = 0,
  /**
   * @brief A required pointer argument was NULL.
   */
  MAYON_RESULT_NULL_ARG = 36865,
  /**
   * @brief A backend failed to initialize due to a platform or loader error.
   */
  MAYON_RESULT_BACKEND_LOAD_ERROR = 40961,
  /**
   * @brief Unsupported target windowing platform
   */
  MAYON_RESULT_UNSUPPORTED_PLATFORM_ERROR = 40962,
  /**
   * @brief Vulkan could not be loaded or initialized.
   */
  MAYON_RESULT_VULKAN_LOAD_ERROR = 45057,
  /**
   * @brief An unspecified internal error occurred.
   */
  MAYON_RESULT_UNKNOWN_ERROR = 65535,
};
#ifndef __cplusplus
typedef uint16_t MynFallibleResult;
#endif // __cplusplus

/**
 * @brief Vulkan version structure.
 */
typedef struct MynVkVersion {
  /**
   * @brief Major version number (e.g., 1 in Vulkan 1.3.0).
   */
  uint32_t major;
  /**
   * @brief Minor version number (e.g., 3 in Vulkan 1.3.0).
   */
  uint32_t minor;
  /**
   * @brief Patch version number (e.g., 0 in Vulkan 1.3.0).
   */
  uint32_t patch;
} MynVkVersion;

/**
 * @brief Vulkan backend initialization parameters.
 *
 * @note All pointer fields are borrowed for the duration of the call.
 */
typedef struct MynVkBackendParams {
  /**
   * @brief Optional null-terminated UTF-8 application name.
   */
  const char *application_name;
  /**
   * @brief Application version.
   */
  struct MynVkVersion application_version;
  /**
   * @brief Optional null-terminated UTF-8 engine name.
   */
  const char *engine_name;
  /**
   * @brief Engine version.
   */
  struct MynVkVersion engine_version;
} MynVkBackendParams;

typedef struct MynCustomAllocator {
  uint8_t *(*pfn_allocate)(uintptr_t size, uintptr_t alignment);
  void (*pfn_deallocate)(uint8_t *ptr);
  uint8_t *(*pfn_reallocate)(uint8_t *ptr, uintptr_t new_size, uintptr_t alignment);
} MynCustomAllocator;

/**
 * @brief Opaque Mayon instance handle.
 *
 * @note Instances are reference-counted internally.
 */
typedef struct MynInstance {
  uintptr_t _0;
} MynInstance;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * @brief Creates a new Mayon instance using the Vulkan API as backend.
 *
 * @param params Pointer to a \c MynVkBackendParams structure. Must not be \c NULL.
 * @param allocator use to set a custom allocator.
 * @param out_instance Pointer to storage that will receive the created Instance. Must not be \c NULL.
 *
 * @return \c MAYON_RESULT_OK on success.
 * @return A non-zero \c MynFallibleResult error code on failure.
 * @return \c MAYON_RESULT_NULL_ARG if \p params or \p out_instance is \c NULL.
 *
 * @par Behavior
 * On success, a valid Instance handle is written to \p out_instance.
 * On failure, *\p out_instance remains unchanged and an error message is stored
 * (retrievable via \c mayon_last_error_message()).
 *
 * @par Requirements
 * - \p params must point to a valid \c MynVkBackendParams structure.
 * - \p out_instance must point to writable, properly aligned memory.
 * - All string pointers within \p params must be valid null-terminated UTF-8 C strings.
 */
MynFallibleResult mayon_new_instance_on_vulkan(const struct MynVkBackendParams *params,
                                               const struct MynCustomAllocator *allocator,
                                               struct MynInstance *out_instance);

/**
 * @brief Releases a Mayon instance.
 *
 * @param instance Pointer to the Mayon instance to release.
 *
 * @note
 *
 * \par Safety
 *
 * - Instances are internally reference-counted. Releasing the same instance
 *   multiple times may cause unintended deallocation once the reference count
 *   reaches zero.
 *
 * - Passing a null pointer has no effect.
 */
void mayon_drop_instance(struct MynInstance *instance);

/**
 * @brief Returns the last error message for the calling thread.
 *
 * @returns Pointer to a null-terminated UTF-8 string describing the last error.
 * @returns NULL if no error is currently set.
 *
 * @par Lifetime and ownership:
 * - The returned pointer must NOT be freed.
 * - The pointer remains valid until the next error is set on the same thread.
 *
 * @par Threading:
 * - Error messages are stored per-thread.
 * - Calling this function does not affect other threads.
 */
const char *mayon_last_error_message(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* MAYON_API_H */
