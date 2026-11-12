use std::path::PathBuf;
use crate::parser::flags::new_flags::NewTemplates;
use crate::logs;
use super::templates::react::generate_react;
use super::templates::nest::generate_nest;

fn run_generator(name: &str, template: NewTemplates, base_path: PathBuf) -> Result<(), std::io::Error> {
    let status = match template {
        NewTemplates::React => generate_react(name, base_path),
        NewTemplates::NestJs => generate_nest(name, base_path),
    }?;

    if !status.success() {
        logs::error(&format!("Project {} was not initialised!", name));
        return Err(std::io::Error::other("Node package manager could not run the command"));
    }

    return Ok(());
}

pub fn execute(
    name: String,
    path: Option<PathBuf>,
    template: Option<NewTemplates>,
) -> Result<(), std::io::Error> {
    let base_path = path.unwrap_or_else(|| PathBuf::from("."));
    let project_path = base_path.join(&name);
    let project_template = template.unwrap_or(NewTemplates::React);
    logs::log(&format!("Starting the creation of {:?} project\nName of project: {}\nPath of the project: {}", project_template, String::from(&name), project_path.display()));

    if project_path.exists() {
        logs::warn(&format!("The current path '{}' already exists", project_path.display()));
        return Ok(());
    }

    run_generator(&name, project_template, base_path.clone())?;

    logs::log(&format!("Project has been created in '{}'", project_path.display()));

    return Ok(());
}