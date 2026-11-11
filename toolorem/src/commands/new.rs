use std::path::PathBuf;

use crate::parser::flags::new_flags::NewTemplates;
use crate::logs::logs;

pub fn execute(
    name: String,
    path: Option<PathBuf>,
    template: Option<NewTemplates>,
) {
    let project_path = path.unwrap_or_else(|| PathBuf::from("."));
    let project_template = template.unwrap_or(NewTemplates::React);

    logs::log(&format!("Создаем проект: {}", name));
    logs::warn(&format!("Путь: {:?}", project_path));
    logs::error(&format!("По шаблону: {:?}", project_template));
}