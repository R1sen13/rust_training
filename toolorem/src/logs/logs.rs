const LOG_STYLE: &str = "\x1b[92m";
const WARN_STYLE: &str = "\x1b[93m";
const DEBUG_STYLE: &str = "\x1b[95m";
const ERROR_STYLE: &str = "\x1b[91m";
const RESET_STYLE: &str = "\x1b[0m";

fn write_log(style: &str, level: &str, message: &str) {
    let label = format!("{}", level);
    println!("{style}{label}{RESET_STYLE}: {message}");
}

pub fn log(message: &str) {
    write_log(LOG_STYLE, "info", message);
}

pub fn warn(message: &str) {
    write_log(WARN_STYLE, "warning", message);
}

pub fn debug(message: &str) {
    write_log(DEBUG_STYLE, "debug", message);
}

pub fn error(message: &str) {
    write_log(ERROR_STYLE, "error", message);
}