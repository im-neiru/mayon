use core::fmt::Arguments;

pub trait Logger {
    fn log(&mut self, level: Level, target: Target, args: Arguments);
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, strum::Display)]
pub enum Level {
    #[strum(to_string = "Error")]
    Error = 1,

    #[strum(to_string = "Warn")]
    Warn = 2,

    #[strum(to_string = "Info")]
    Info = 3,

    #[strum(to_string = "Debug")]
    Debug = 4,

    #[strum(to_string = "Trace")]
    Trace = 5,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, strum::Display, strum::IntoStaticStr)]
pub enum Target {
    #[strum(serialize = "mayon::backend")]
    Backend,
}

#[derive(Default)]
pub struct DefaultLogger;

#[derive(Default)]
pub struct QuietLogger;

impl Logger for DefaultLogger {
    #[inline]
    #[track_caller]
    fn log(&mut self, level: Level, target: Target, args: Arguments) {
        let level = match level {
            Level::Error => log::Level::Error,
            Level::Warn => log::Level::Warn,
            Level::Info => log::Level::Info,
            Level::Debug => log::Level::Debug,
            Level::Trace => log::Level::Trace,
        };

        let location = core::panic::Location::caller();

        let record = log::Record::builder()
            .level(level)
            .target(Into::<&'static str>::into(target))
            .file_static(Some(location.file()))
            .line(Some(location.line()))
            .args(args)
            .build();

        log::logger().log(&record);
    }
}

impl Logger for QuietLogger {
    #[inline(always)]
    fn log(&mut self, _: Level, _: Target, _: Arguments) {
        /* Be quiet */
    }
}

#[macro_export]
macro_rules! error {
     ($logger:expr, $target:expr, $($arg:tt)*) => {{
        $logger.log(
            $crate::logger::Level::Error,
            $target,
            core::format_args!($($arg)*),
        );
    }};
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
          $logger.log(
            $crate::logger::Level::Warn,
            $target,
            core::format_args!($($arg)*),
        );
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
          $logger.log(
            $crate::logger::Level::Info,
            $target,
            core::format_args!($($arg)*),
        );
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
          $logger.log(
            $crate::logger::Level::Debug,
            $target,
            core::format_args!($($arg)*),
        );
    };
}

#[macro_export]
macro_rules! trace {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
           $logger.log(
            $crate::logger::Level::Trace,
            $target,
            core::format_args!($($arg)*),
        );
    };
}
