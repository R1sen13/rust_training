mod parser;
mod commands;
mod logs;

use parser::{Cli, Commands};
use clap::Parser;
use commands::new;

fn handle_error<E>(result: Result<(), E>) where E: std::fmt::Display {
    if let Err(err) = result {
        logs::error(&format!("Command has resulted in error: {}", err.to_string()));
        std::process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name, path, template } => {
            new::execute(name, path, template)
        },
    };

    handle_error(result);
}
