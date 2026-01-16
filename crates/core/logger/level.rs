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
