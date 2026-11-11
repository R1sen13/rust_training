mod parser;
use parser::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => println!("Создаем проект: {}", name),
    }
}
