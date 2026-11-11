use clap::{Parser, Subcommand};
use std::path::PathBuf;

use super::flags::new_flags::NewTemplates;


#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        name: String,

        #[arg(long)]
        path: Option<PathBuf>,

        #[arg(long)]
        template: Option<NewTemplates>,
    }
}