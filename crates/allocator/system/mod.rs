#[cfg(target_os = "windows")]
mod windows;

pub struct System;

#[cfg(feature = "for_c_api")]
pub use windows::c_api;
