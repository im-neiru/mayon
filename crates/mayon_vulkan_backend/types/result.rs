use core::mem::transmute;

#[cfg(feature = "error_location")]
use crate::VulkanFunctionName;
use crate::{Result, VulkanError, VulkanErrorKind::FunctionReturn};

#[repr(i32)]
#[allow(unused)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VkResult {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    HostMemory = -1,
    DeviceMemory = -2,
    InitializationFailed = -3,
    DeviceLost = -4,
    MemoryMapFailed = -5,
    LayerNotPresent = -6,
    ExtensionNotPresent = -7,
    FeatureNotPresent = -8,
    IncompatibleDriver = -9,
    TooManyObjects = -10,
    FormatNotSupported = -11,
    FragmentedPool = -12,
    Unknown = -13,
    ValidationFailed = -1000011001,
    OutOfPoolMemory = -1000069000,
    InvalidExternalHandle = -1000072003,
    InvalidOpaqueCaptureAddress = -1000257000,
    Fragmentation = -1000161000,
    PipelineCompileRequired = 1000297000,
    NotPermitted = -1000174001,
    SurfaceLostKhr = -1000000000,
    NativeWindowInUseKhr = -1000000001,
    SuboptimalKhr = 1000001003,
    OutOfDateKhr = -1000001004,
    IncompatibleDisplayKhr = -1000003001,
    InvalidShaderNv = -1000012000,
    ImageUsageNotSupportedKhr = -1000023000,
    VideoPictureLayoutNotSupportedKhr = -1000023001,
    VideoProfileOperationNotSupportedKhr = -1000023002,
    VideoProfileFormatNotSupportedKhr = -1000023003,
    VideoProfileCodecNotSupportedKhr = -1000023004,
    VideoStdVersionNotSupportedKhr = -1000023005,
    InvalidDrmFormatModifierPlaneLayoutExt = -1000158000,
    PresentTimingQueueFullExt = -1000208000,
    FullScreenExclusiveModeLostExt = -1000255000,
    ThreadIdleKhr = 1000268000,
    ThreadDoneKhr = 1000268001,
    OperationDeferredKhr = 1000268002,
    OperationNotDeferredKhr = 1000268003,
    InvalidVideoStdParametersKhr = -1000299000,
    CompressionExhaustedExt = -1000338000,
    IncompatibleShaderBinaryExt = 1000482000,
    PipelineBinaryMissingKhr = 1000483000,
    NotEnoughSpaceKhr = -1000483000,
}

impl VkResult {
    #[cfg(feature = "error_location")]
    #[track_caller]
    #[inline(always)]
    pub(crate) fn into_result<T>(
        self,
        name: VulkanFunctionName,
        success: impl FnOnce() -> T,
    ) -> Result<T> {
        if Self::Success == self {
            Ok(success())
        } else {
            Err(VulkanError {
                kind: FunctionReturn {
                    name,
                    code: unsafe { transmute::<Self, ReturnCode>(self) },
                },
                location: core::panic::Location::caller(),
            })
        }
    }

    #[cfg(not(feature = "error_location"))]
    #[inline(always)]
    pub(crate) fn into_result<T>(
        self,
        name: VulkanFunctionName,
        success: impl FnOnce() -> T,
    ) -> Result<T> {
        if Self::Success == self {
            Ok(success())
        } else {
            Err(VulkanError {
                kind: FunctionReturn {
                    name,
                    code: unsafe { transmute::<Self, ReturnCode>(self) },
                },
            })
        }
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, thiserror::Error)]
pub enum ReturnCode {
    #[error("The operation is not ready.")]
    NotReady = 1,

    #[error("The operation timed out.")]
    Timeout = 2,

    #[error("The event is set.")]
    EventSet = 3,

    #[error("The event is reset.")]
    EventReset = 4,

    #[error("The operation is incomplete.")]
    Incomplete = 5,

    #[error("The system ran out of host memory.")]
    HostMemory = -1,

    #[error("The system ran out of device memory.")]
    DeviceMemory = -2,

    #[error("Initialization failed.")]
    InitializationFailed = -3,

    #[error("The device was lost.")]
    DeviceLost = -4,

    #[error("Memory mapping failed.")]
    MemoryMapFailed = -5,

    #[error("The requested layer is not present.")]
    LayerNotPresent = -6,

    #[error("The requested extension is not present.")]
    ExtensionNotPresent = -7,

    #[error("The requested feature is not present.")]
    FeatureNotPresent = -8,

    #[error("The driver is incompatible.")]
    IncompatibleDriver = -9,

    #[error("Too many objects have been created.")]
    TooManyObjects = -10,

    #[error("The requested format is not supported.")]
    FormatNotSupported = -11,

    #[error("The memory pool is fragmented.")]
    FragmentedPool = -12,

    #[error("An unknown error occurred.")]
    Unknown = -13,

    #[error("Validation failed.")]
    ValidationFailed = -1000011001,

    #[error("The pool is out of memory.")]
    OutOfPoolMemory = -1000069000,

    #[error("The external handle is invalid.")]
    InvalidExternalHandle = -1000072003,

    #[error("The opaque capture address is invalid.")]
    InvalidOpaqueCaptureAddress = -1000257000,

    #[error("Memory fragmentation occurred.")]
    Fragmentation = -1000161000,

    #[error("Pipeline compilation is required.")]
    PipelineCompileRequired = 1000297000,

    #[error("The operation is not permitted.")]
    NotPermitted = -1000174001,

    #[error("The rendering surface was lost.")]
    SurfaceLostKhr = -1000000000,

    #[error("The native window is already in use.")]
    NativeWindowInUseKhr = -1000000001,

    #[error("The swapchain is suboptimal.")]
    SuboptimalKhr = 1000001003,

    #[error("The swapchain is out of date.")]
    OutOfDateKhr = -1000001004,

    #[error("The display configuration is incompatible.")]
    IncompatibleDisplayKhr = -1000003001,

    #[error("The shader is invalid.")]
    InvalidShaderNv = -1000012000,

    #[error("The image usage is not supported.")]
    ImageUsageNotSupportedKhr = -1000023000,

    #[error("The video picture layout is not supported.")]
    VideoPictureLayoutNotSupportedKhr = -1000023001,

    #[error("The video profile operation is not supported.")]
    VideoProfileOperationNotSupportedKhr = -1000023002,

    #[error("The video profile format is not supported.")]
    VideoProfileFormatNotSupportedKhr = -1000023003,

    #[error("The video profile codec is not supported.")]
    VideoProfileCodecNotSupportedKhr = -1000023004,

    #[error("The video standard version is not supported.")]
    VideoStdVersionNotSupportedKhr = -1000023005,

    #[error("The DRM format modifier plane layout is invalid.")]
    InvalidDrmFormatModifierPlaneLayoutExt = -1000158000,

    #[error("The present timing queue is full.")]
    PresentTimingQueueFullExt = -1000208000,

    #[error("The full-screen exclusive mode was lost.")]
    FullScreenExclusiveModeLostExt = -1000255000,

    #[error("The thread is idle.")]
    ThreadIdleKhr = 1000268000,

    #[error("The thread has completed its work.")]
    ThreadDoneKhr = 1000268001,

    #[error("The operation was deferred.")]
    OperationDeferredKhr = 1000268002,

    #[error("The operation was not deferred.")]
    OperationNotDeferredKhr = 1000268003,

    #[error("The video standard parameters are invalid.")]
    InvalidVideoStdParametersKhr = -1000299000,

    #[error("Compression resources have been exhausted.")]
    CompressionExhaustedExt = -1000338000,

    #[error("The shader binary is incompatible.")]
    IncompatibleShaderBinaryExt = 1000482000,

    #[error("The pipeline binary is missing.")]
    PipelineBinaryMissingKhr = 1000483000,

    #[error("There is not enough space available.")]
    NotEnoughSpaceKhr = -1000483000,
}
