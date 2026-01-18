#[cfg(target_env = "msvc")]
mod msvcrt;

#[cfg(target_env = "msvc")]
pub use msvcrt::*;
