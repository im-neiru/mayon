pub trait Logger {
    fn log(&mut self, level: Level, message: &str);
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
    fn log(&mut self, _level: Level, _message: &str) {
        todo!()
    }
}

impl Logger for QuietLogger {
    #[inline(always)]
    fn log(&mut self, _: Level, _: &str) {
        /* Be quiet */
    }
}
