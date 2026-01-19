// Octal layout for FallibleResult (u16):
//
// 0oE_C_SS_DDD
// ──────────────
// E   = error flag (1 = error, 0 = success)
// C   = class (API misuse, platform, backend, internal, etc.)
// SS  = subsystem (e.g., 0 = generic, 00 = Vulkan, etc.)
// DDD = detail (specific error code)
//
// Each group is separated by underscores for readability.
// All error codes have the highest bit set implicitly in the layout.

/// Numeric result codes returned by Mayon C API functions.
///
/// The value layout is implementation-defined but stable.
/// Applications should compare against the named constants.
#[repr(u16)]
#[allow(clippy::unusual_byte_groupings)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MynFallibleResult {
    /// @brief The operation succeeded.
    MAYON_RESULT_OK = 0o0_0_00_00, // success

    // --- Client / API misuse (class=01) ---
    /// @brief A required pointer argument was NULL.
    MAYON_RESULT_NULL_ARG = 0o1_1_00_01, // subsystem=0, detail=1

    // --- Platform General Error (class=02) ---
    /// @brief A backend failed to initialize due to a platform or loader error.
    MAYON_RESULT_BACKEND_ALLOCATION = 0o1_2_00_01, // subsystem=0, detail=1
    MAYON_RESULT_BACKEND_LOAD_ERROR = 0o1_2_00_02, // subsystem=0, detail=2

    /// @brief Unsupported target windowing platform
    MAYON_RESULT_UNSUPPORTED_PLATFORM_ERROR = 0o1_2_00_03, // subsystem=0, detail=3

    // --- Backend Graphics API Errors (class=03) ---
    /// @brief Vulkan could not be loaded or initialized.
    MAYON_RESULT_VULKAN_LOAD_ERROR = 0o1_3_00_01, // subsystem=00 (Vulkan), detail=1

    /// @brief An unspecified internal error occurred.
    MAYON_RESULT_UNKNOWN_ERROR = 0o1_7_77_77, // catch-all
}
