use std::process::{Command, ExitStatus};
use std::path::PathBuf;
use crate::logs;

pub fn generate_nest(name: &str, base_path: PathBuf) -> Result<ExitStatus, std::io::Error> {
    logs::debug("Starting NestJs initialiser");
    //TODO make linux accessibility
    Command::new("npx.cmd")
        .args(["@nestjs/cli", "new", &name, "--package-manager", "npm", "--skip-git"])
        .current_dir(base_path)
        .status()
}