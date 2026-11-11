use std::path::PathBuf;

use crate::parser::flags::new_flags::NewTemplates;
use crate::logs;

pub fn execute(
    name: String,
    path: Option<PathBuf>,
    template: Option<NewTemplates>,
) -> Result<(), std::io::Error> {
    let base_path = path.unwrap_or_else(|| PathBuf::from("."));
    let project_path = base_path.join(&name);
    logs::log(&format!("Starting the creation of {:?} project\nName of project: {}\nPath of the project: {}", template, String::from(&name), project_path.display()));

    if project_path.exists() {
        logs::warn(&format!("The current path '{}' already contains files", project_path.display()));
        return Ok(());
    }
    std::fs::create_dir_all(&project_path)?;
    logs::log(&format!("Project has being created in '{}'", project_path.display()));

    let project_template = template.unwrap_or(NewTemplates::React);
    return Ok(());
}