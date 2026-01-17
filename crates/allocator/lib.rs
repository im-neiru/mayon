#![no_std]

mod allocator;
mod global;

pub use allocator::{AllocError, Allocator};
pub use global::System;
