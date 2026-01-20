mod backend;
mod context;
mod errors;
mod fn_table;
mod types;

pub use errors::{Error, ErrorKind, Result};

pub use backend::{VulkanBackend, VulkanBackendParams, VulkanVersion};
pub use context::VulkanContext;
pub use types::ReturnCode;

pub type Instance<'a, L = mayon_core::logger::DefaultLogger, A = allocator::System> =
    mayon_core::Instance<VulkanBackend<'a, L, A>, L, A>;
