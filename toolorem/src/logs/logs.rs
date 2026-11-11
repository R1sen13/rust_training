const LOG_STYLE: &str = "\x1b[42m\x1b[37m";
const WARN_STYLE: &str = "\x1b[43m\x1b[37m";
const DEBUG_STYLE: &str = "\x1b[44m\x1b[37m";
const ERROR_STYLE: &str = "\x1b[41m\x1b[37m";
const RESET_STYLE: &str = "\x1b[0m";

pub fn log(message: &str) {
    let label = format!("{:^3}", "INF");
    let style = LOG_STYLE;
    println!("{style}{label}{RESET_STYLE} {message}");
}

pub fn warn(message: &str) {
    let label = format!("{:^3}", "WRN");
    let style = WARN_STYLE;
    println!("{style}{label}{RESET_STYLE} {message}");
}

pub fn debug(message: &str) {
    let label = format!("{:^3}", "DBG");
    let style = DEBUG_STYLE;
    println!("{style}{label}{RESET_STYLE} {message}");
}

pub fn error(message: &str) {
    let label = format!("{:^3}", "ERR");
    let style = ERROR_STYLE;
    println!("{style}{label}{RESET_STYLE} {message}");
}