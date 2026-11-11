const LOG_STYLE: &str = "\x1b[42m\x1b[37m";
const WARN_STYLE: &str = "\x1b[43m\x1b[37m";
const DEBUG_STYLE: &str = "\x1b[44m\x1b[37m";
const ERROR_STYLE: &str = "\x1b[41m\x1b[37m";
const RESET_STYLE: &str = "\x1b[0m";

fn write_log(style: &str, level: &str, message: &str) {
    let label = format!("{:^3}", level);
    println!("{style}{label}{RESET_STYLE} {message}");
}

pub fn log(message: &str) {
    write_log(LOG_STYLE, "INF", message);
}

pub fn warn(message: &str) {
    write_log(WARN_STYLE, "WRN", message);
}

pub fn debug(message: &str) {
    write_log(DEBUG_STYLE, "DBG", message);
}

pub fn error(message: &str) {
    write_log(ERROR_STYLE, "ERR", message);
}