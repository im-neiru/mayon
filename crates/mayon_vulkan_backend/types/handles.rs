use helper_macros::vk_handle;

#[vk_handle(usize)]
pub struct Instance;

#[vk_handle(u64)]
pub struct Surface;

#[vk_handle(usize)]
pub struct PhysicalDevice;

#[vk_handle(usize)]
pub struct Device;

#[vk_handle(u64)]
pub struct Image;

#[vk_handle(u64)]
pub struct ImageView;

#[vk_handle(usize)]
pub struct Queue;

#[vk_handle(usize)]
pub struct CommandBuffer;

#[vk_handle(u64)]
pub struct Semaphore;

#[vk_handle(u64)]
pub struct Fence;

#[vk_handle(u64)]
pub struct DeviceMemory;

#[vk_handle(u64)]
pub struct Buffer;

#[vk_handle(u64)]
pub struct BufferView;

#[vk_handle(u64)]
pub struct ShaderModule;

#[vk_handle(u64)]
pub struct Pipeline;

#[vk_handle(u64)]
pub struct PipelineLayout;

#[vk_handle(u64)]
pub struct Sampler;

#[vk_handle(u64)]
pub struct DescriptorSet;

#[vk_handle(u64)]
pub struct DescriptorSetLayout;

#[vk_handle(u64)]
pub struct DescriptorPool;

#[vk_handle(u64)]
pub struct CommandPool;

#[vk_handle(u64)]
pub struct RenderPass;

#[vk_handle(u64)]
pub struct Framebuffer;

#[vk_handle(u64)]
pub struct Swapchain;

#[vk_handle(u64)]
pub struct DebugUtilsMessenger;
