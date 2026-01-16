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
