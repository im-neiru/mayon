mod level;
mod loggers;
mod macros;
mod target;

pub use level::Level;
pub use loggers::*;
pub use target::Target;

use core::fmt::Arguments;

pub trait Logger {
    fn log(&mut self, level: Level, target: Target, args: Arguments);
}
