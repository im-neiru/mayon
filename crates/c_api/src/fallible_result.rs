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
#[repr(u16)]
#[allow(clippy::unusual_byte_groupings)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FallibleResult {
    Ok = 0o0_0_00_00, // success

    // Client / API misuse (class=01)
    NullPointerParam = 0o1_1_00_01, // subsystem=0, detail=1

    // Platform General Error (class=02)
    BackendLoadFailed = 0o1_2_00_01, // subsystem=0, detail=1

    // Backend Graphics API Errors (class=03)
    VulkanFunctionError = 0o1_3_00_01, // subsystem=00 (Vulkan), detail=1

    UnknownError = 0o1_7_77_77, // catch-all
}
