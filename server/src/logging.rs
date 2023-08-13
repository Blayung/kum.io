use colored::Colorize;
use crate::config;

// TODO: Write the logs to a file

// Printing
pub fn info(message: &str) {
    println!("{} {}", "[INFO]".green().bold(), message);
}
pub fn _info(message: String) { info(&message); }

pub fn warning(message: &str) {
    println!("{} {}", "[WARN]".yellow().bold(), message);
}
pub fn _warning(message: String) { warning(&message); }

pub fn error(message: &str) {
    println!("{} {}", "[ERROR]".bright_red().bold(), message);
}
pub fn _error(message: String) { error(&message); }

pub fn fatal(message: &str) -> ! {
    println!("{} {}", "[FATAL]".red().bold(), message);
    panic!();
}
pub fn _fatal(message: String) -> ! { fatal(&message); }

pub fn extra(message: &str) {
    if config::get().log_level > 0 {
        println!("{} {}", "[EXTRA]".bright_green().bold(), message);
    }
}
pub fn _extra(message: String) { extra(&message); }

pub fn debug(message: &str) {
    if config::get().log_level > 1 {
        println!("{} {}", "[DEBUG]".purple().bold(), message);
    }
}
pub fn _debug(message: String) { debug(&message); }

// Handling unwraps
pub fn unwrap_opt<T>(option: Option<T>) -> T {
    match option {
        Some(t) => t,
        None => fatal("called `logging::unwrap_opt()` on a `None` value")
    }
}

pub fn unwrap_res<T,E>(result: Result<T,E>) -> T
where
    E: std::fmt::Debug
{
    match result {
        Ok(t) => t,
        Err(e) => _fatal(format!("called `logging::unwrap_res()` on an `Err` value: {:?}", &e))
    }
}
