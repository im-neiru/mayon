use super::*;

#[derive(Default)]
pub struct DefaultLogger;

#[derive(Default)]
pub struct QuietLogger;

impl Logger for DefaultLogger {
    /// Logs a message to the global logger, preserving the call-site file, line, and provided target.
    ///
    /// Constructs a `log::Record` from the given `level`, `target`, caller location, and `args`, then forwards it to the global logger.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mayon_core::logger::{DefaultLogger, Logger, Target, Level};
    ///
    /// let mut logger = DefaultLogger::default();
    /// logger.log(Level::Info, Target::Backend, format_args!("initialized: {}", true));
    ///
    #[inline]
    #[track_caller]
    fn log(&self, level: Level, target: Target, args: Arguments) {
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
    /// Silences all logging; calls to this method have no observable effect.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mayon_core::logger::{QuietLogger, Logger, Target, Level};
    ///
    /// let mut logger = QuietLogger::default();
    /// logger.log(Level::Info, Target::Backend, format_args!("initialized: {}", true));
    ///
    #[inline(always)]
    fn log(&self, _: Level, _: Target, _: Arguments) {
        /* Be quiet */
    }
}
