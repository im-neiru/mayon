use helper_macros::vk_handle;

/// Opaque handle to a Vulkan instance.
///
/// Represents a `VkInstance`, which is the connection between your application
/// and the Vulkan implementation. Dispatchable handle (pointer-sized).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html
#[vk_handle(usize)]
pub struct Instance;

/// Opaque handle to a Vulkan surface.
///
/// Represents a `VkSurfaceKHR`, which is an abstraction of a native platform
/// surface for presenting rendered images. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html
#[vk_handle(u64)]
pub struct Surface;

/// Opaque handle to a Vulkan physical device.
///
/// Represents a `VkPhysicalDevice`, which is a single GPU or other Vulkan-capable
/// device in the system. Dispatchable handle (pointer-sized).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html
#[vk_handle(usize)]
pub struct PhysicalDevice;

/// Opaque handle to a Vulkan logical device.
///
/// Represents a `VkDevice`, which is the primary interface for interacting with
/// a physical device. Dispatchable handle (pointer-sized).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html
#[vk_handle(usize)]
pub struct Device;

/// Opaque handle to a Vulkan image.
///
/// Represents a `VkImage`, which is a multidimensional array of texel data.
/// Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html
#[vk_handle(u64)]
pub struct Image;

/// Opaque handle to a Vulkan image view.
///
/// Represents a `VkImageView`, which describes how to access an image and which
/// portion of the image to access. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html
#[vk_handle(u64)]
pub struct ImageView;

/// Opaque handle to a Vulkan queue.
///
/// Represents a `VkQueue`, which is an interface for submitting command buffers
/// to a device. Dispatchable handle (pointer-sized).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html
#[vk_handle(usize)]
pub struct Queue;

/// Opaque handle to a Vulkan command buffer.
///
/// Represents a `VkCommandBuffer`, which records commands for execution on a queue.
/// Dispatchable handle (pointer-sized).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html
#[vk_handle(usize)]
pub struct CommandBuffer;

/// Opaque handle to a Vulkan semaphore.
///
/// Represents a `VkSemaphore`, which is a synchronization primitive for
/// GPU-GPU synchronization. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html
#[vk_handle(u64)]
pub struct Semaphore;

/// Opaque handle to a Vulkan fence.
///
/// Represents a `VkFence`, which is a synchronization primitive for
/// CPU-GPU synchronization. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html
#[vk_handle(u64)]
pub struct Fence;

/// Opaque handle to Vulkan device memory.
///
/// Represents a `VkDeviceMemory` allocation, which is a block of memory
/// allocated from a device heap. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html
#[vk_handle(u64)]
pub struct DeviceMemory;

/// Opaque handle to a Vulkan buffer.
///
/// Represents a `VkBuffer`, which is a linear array of data for uniform buffers,
/// vertex buffers, etc. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html
#[vk_handle(u64)]
pub struct Buffer;

/// Opaque handle to a Vulkan buffer view.
///
/// Represents a `VkBufferView`, which describes how to interpret buffer data
/// for image operations. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html
#[vk_handle(u64)]
pub struct BufferView;

/// Opaque handle to a Vulkan shader module.
///
/// Represents a `VkShaderModule`, which contains shader code in SPIR-V format.
/// Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html
#[vk_handle(u64)]
pub struct ShaderModule;

/// Opaque handle to a Vulkan pipeline.
///
/// Represents a `VkPipeline`, which encapsulates the entire graphics or compute
/// pipeline state. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html
#[vk_handle(u64)]
pub struct Pipeline;

/// Opaque handle to a Vulkan pipeline layout.
///
/// Represents a `VkPipelineLayout`, which describes the set of descriptor sets
/// and push constants used by a pipeline. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html
#[vk_handle(u64)]
pub struct PipelineLayout;

/// Opaque handle to a Vulkan sampler.
///
/// Represents a `VkSampler`, which controls how texture sampling operations
/// are performed. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html
#[vk_handle(u64)]
pub struct Sampler;

/// Opaque handle to a Vulkan descriptor set.
///
/// Represents a `VkDescriptorSet`, which binds resources (buffers, images, etc.)
/// to shader stages. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html
#[vk_handle(u64)]
pub struct DescriptorSet;

/// Opaque handle to a Vulkan descriptor set layout.
///
/// Represents a `VkDescriptorSetLayout`, which defines the structure of a
/// descriptor set. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html
#[vk_handle(u64)]
pub struct DescriptorSetLayout;

/// Opaque handle to a Vulkan descriptor pool.
///
/// Represents a `VkDescriptorPool`, which allocates descriptor sets.
/// Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html
#[vk_handle(u64)]
pub struct DescriptorPool;

/// Opaque handle to a Vulkan command pool.
///
/// Represents a `VkCommandPool`, which manages memory for command buffer recording.
/// Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html
#[vk_handle(u64)]
pub struct CommandPool;

/// Opaque handle to a Vulkan render pass.
///
/// Represents a `VkRenderPass`, which describes the structure and dependencies
/// of rendering operations. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html
#[vk_handle(u64)]
pub struct RenderPass;

/// Opaque handle to a Vulkan framebuffer.
///
/// Represents a `VkFramebuffer`, which is a collection of attachments used
/// in a render pass. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html
#[vk_handle(u64)]
pub struct Framebuffer;

/// Opaque handle to a Vulkan swapchain.
///
/// Represents a `VkSwapchainKHR`, which is a chain of images for presenting
/// to a surface. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html
#[vk_handle(u64)]
pub struct Swapchain;

/// Opaque handle to a Vulkan debug messenger.
///
/// Represents a `VkDebugUtilsMessengerEXT`, which receives debug callbacks
/// from the validation layers. Non-dispatchable handle (64-bit).
///
/// Vulkan Documentation Reference: https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html
#[vk_handle(u64)]
pub struct DebugUtilsMessenger;
