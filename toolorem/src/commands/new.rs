use std::path::PathBuf;

use crate::parser::flags::new_flags::NewTemplates;

pub fn execute(
    name: String,
    path: Option<PathBuf>,
    template: Option<NewTemplates>,
) {
    let project_path = path.unwrap_or_else(|| PathBuf::from("."));
    let project_template = template.unwrap_or(NewTemplates::React);

    println!("Создаем проект: {}", name);
    println!("Путь: {:?}", project_path);
    println!("По шаблону: {:?}", project_template);
}