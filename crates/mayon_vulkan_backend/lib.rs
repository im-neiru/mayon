mod backend;
mod context;
mod errors;
mod fn_table;
mod types;

pub use errors::{Error, ErrorKind, Result};
pub use fn_table::VulkanFunctionName;

pub use backend::{VulkanBackend, VulkanBackendParams, VulkanVersion};
pub use context::VulkanContext;
pub use types::ReturnCode;

pub type Instance<'a, L = mayon_core::logger::DefaultLogger, A = allocator::System> =
    mayon_core::Instance<VulkanBackend<'a, L, A>, L, A>;
pub type Context<'a, L = mayon_core::logger::DefaultLogger, A = allocator::System> =
    mayon_core::Context<VulkanBackend<'a, L, A>, L, A>;
