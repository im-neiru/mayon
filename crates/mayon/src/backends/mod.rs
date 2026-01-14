mod target_platform;
mod traits;

pub mod vulkan;

pub use target_platform::TargetPlatform;
pub(crate) use traits::{Backend, CreateBackend};
