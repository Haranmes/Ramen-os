pub static mut CURSOR_X: usize = 0;
pub static mut CURSOR_Y: usize = 0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    None = 0,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic,
}

// Define a macro that accepts the log level and message.
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        let (label, color_code, show_location) = match $level {
            crate::log::LogLevel::None => ("NONE", 0xffffff, false),
            crate::log::LogLevel::Trace => ("TRACE", 0x00cc00, false),
            crate::log::LogLevel::Debug => ("DEBUG", 0xff0000, false),
            crate::log::LogLevel::Info  => ("INFO", 0xffff00, false),

            crate::log::LogLevel::Warn => {
                #[cfg(debug_assertions)]
                { ("WARN ", 0xff0000, true) }

                #[cfg(not(debug_assertions))]
                { ("WARN ", 0xff0000, false) }
            }

            crate::log::LogLevel::Error => ("ERROR", 0xcc0000, true),
            crate::log::LogLevel::Fatal => ("FATAL", 0xcc0000, true),
            crate::log::LogLevel::Panic => ("PANIC", 0x0066cc, true),
        };

        if show_location {
            crate::write_serial!(
                Some(color_code),
                "[{}] {}:{}: {}",
                label,
                file!(),
                line!(),
                format_args!($($arg)*)
            );
        } else {
            crate::write_serial!(
                Some(color_code),
                "[{}] {}",
                label,
                format_args!($($arg)*)
            );
        }
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        crate::log!($crate::log::LogLevel::Trace, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            crate::log!($crate::log::LogLevel::Debug, $($arg)*);
        }

        #[cfg(not(debug_assertions))]
        {}
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        crate::log!($crate::log::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        crate::log!($crate::log::LogLevel::Warn, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        crate::log!($crate::log::LogLevel::Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        crate::log!($crate::log::LogLevel::Fatal, $($arg)*);
    };
}

#[macro_export]
macro_rules! panic_log {
    ($($arg:tt)*) => {
        crate::log!($crate::log::LogLevel::Panic, $($arg)*);
    };
}