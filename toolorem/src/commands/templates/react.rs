use std::process::{Command, ExitStatus};
use std::path::PathBuf;
use crate::logs;

pub fn generate_react(name: &str, base_path: PathBuf) -> Result<ExitStatus, std::io::Error> {
    logs::debug("Starting React initialiser");
    Command::new("npm.cmd")
        .args(["create", "vite@latest", &name, "--", "--template", "react"])
        .current_dir(&base_path)
        .status()
}