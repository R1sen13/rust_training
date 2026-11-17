use std::process::{Command, ExitStatus};
use std::path::PathBuf;
use crate::logs;

pub struct NestBuilder;

pub fn generate_nest(name: &str, base_path: PathBuf) -> Result<ExitStatus, std::io::Error> {
    logs::debug("Starting NestJs initialiser");
    //TODO make Linux accessibility
    Command::new("npx.cmd")
        .args(["@nestjs/cli", "new", &name, "--package-manager", "npm", "--skip-git"])
        .current_dir(base_path)
        .status()
}

pub fn add_scripts(package_path: PathBuf) -> Result<(), std::io::Error> {
    logs::debug("Starting addition of scripts\nScritps:");
    let content = std::fs::read_to_string(&package_path)?;

    let mut package: serde_json::Value = serde_json::from_str(&content)?;

    let scripts = package
        .get_mut("scripts")
        .and_then(|scripts| scripts.as_object_mut())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "package.json does not contain scripts",
            )
        })?;

    logs::debug("Added 'lint:fix'");
    scripts.insert(
        "lint:fix".to_string(),
        serde_json::Value::String("oxlint --type-aware src/ --fix".to_string()),
    );

    logs::debug("Added 'lint'");
    scripts.insert(
        "lint".to_string(),
        serde_json::Value::String("oxlint --type-aware src/".to_string()),
    );

    logs::debug("Added 'format'");
    scripts.insert(
        "format".to_string(),
        serde_json::Value::String("prettier --check \"src/**/*.ts\"".to_string()),
    );


    logs::debug("Added 'format:fix'");
    scripts.insert(
        "format:fix".to_string(),
        serde_json::Value::String("prettier --write \"src/**/*.ts\"".to_string()),
    );

    logs::debug("Added 'check'");
    scripts.insert(
        "check".to_string(),
        serde_json::Value::String("npm run lint && npm run format && npm run test".to_string()),
    );

    logs::debug("Added 'fix'");
    scripts.insert(
        "fix".to_string(),
        serde_json::Value::String("npm run lint:fix && npm run format:fix".to_string()),
    );

    let formatted = serde_json::to_string_pretty(&package)?;

    std::fs::write(&package_path, formatted)?;
    logs::debug("Package.json written");

    Ok(())
}

impl ProjectBuilder for NestBuilder {
    fn build(&self, name: &str, base_path: PathBuf, package_path: PathBuf) -> Result<ExitStatus, std::io::Error> {
        generate_nest(name, base_path);
        add_scripts(package_path)?;
    }
}