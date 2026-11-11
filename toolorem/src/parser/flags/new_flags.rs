use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum NewTemplates {
    React,
    NestJs,
}