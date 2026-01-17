mod allocator;
mod system;

pub use allocator::{AllocError, Allocator};
pub use system::System;

#[cfg(feature = "for_c_api")]
pub use system::c_api;
