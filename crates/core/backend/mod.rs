mod create_error;
mod target_platform;
mod traits;

pub use create_error::{BackendCreateKind, CreateBackendError};

pub use target_platform::{TargetPlatform, UnsupportedPlatformError};
pub use traits::{Backend, CreateBackend};
