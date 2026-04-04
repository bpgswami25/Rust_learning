//! This crate demonstrates the declarative macro.
//! Here I have created a `log` macro which is accessible outside the crate using `#[macro_export]`.
//! The macro expects at least two parameters: first is the type of log with the prefix `LogType`,
//! for the macro it acts as a simple literal, but we are providing the same type of enum which helps
//! the user to use it from there and all kinds of compilation or typo errors can be avoided with this.

/// Enum representing different log levels.
pub enum LogType {
    Error,
    Fatal,
    Info,
    Debug,
}

/// A declarative macro for logging messages based on log level.
/// 
/// # Examples
/// 
/// ```
/// use declarative_macros::{LogType, log};
/// 
/// log!(LogType::Info, "This is an info message");
/// log!(LogType::Error, "This is an error message");
/// ```
#[macro_export]
macro_rules! log {
    (LogType::Error, $s:expr) => {
        eprintln!("Log::Error: {}", $s);
    };
    (LogType::Fatal, $s:expr) => {
        eprintln!("Log::Fatal: {}", $s);
    };
    (LogType::Info, $s:expr) => {
        println!("Log::Info: {}", $s);
    };
    (LogType::Debug, $s:expr) => {
        println!("Log::Debug: {}", $s);
    };
}

mod doctest{ 
    /// print the info log
    /// ```
    /// use declarative_macros::LogType;
    /// use declarative_macros::log as log;
    /// log!(LogType::Info, "Logging test");
    #[cfg(doctest)]
    fn info_log_test(){}

    /// Compiler error if LogType not proper
    /// ``` compiler_fail
    /// enum Logging_type{Error, info}
    /// use declarative_macros::log as log;
    /// log!(Logging_type::Trace, "Logging test");
    #[cfg(doctest)]
    fn wrong_LogType_log_test(){}
}