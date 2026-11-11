mod parser;
mod commands;

use parser::{Cli, Commands};
use clap::Parser;
use commands::new;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, path, template } => {
            new::execute(name, path, template);
        },
    }
}
