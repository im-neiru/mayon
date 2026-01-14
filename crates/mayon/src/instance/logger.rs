use core::fmt::Arguments;

pub trait Logger {
    fn log(&mut self, level: Level, args: Arguments);
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Level {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

#[derive(Default)]
pub struct DefaultLogger;

#[derive(Default)]
pub struct QuietLogger;

impl Logger for DefaultLogger {
    #[inline]
    #[track_caller]
    fn log(&mut self, level: Level, args: Arguments) {
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
            .file_static(Some(location.file()))
            .line(Some(location.line()))
            .args(args)
            .build();

        log::logger().log(&record);
    }
}

impl Logger for QuietLogger {
    #[inline(always)]
    fn log(&mut self, _: Level, _: Arguments) {
        /* Be quiet */
    }
}
