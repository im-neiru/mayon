use super::*;

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
