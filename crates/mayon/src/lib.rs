#![feature(allocator_api)]
#![feature(once_cell_try)]

pub mod backends;
mod errors;
mod instance;

pub use instance::{Instance, logger};

pub use errors::{HasErrorKind, HasErrorLocation};
