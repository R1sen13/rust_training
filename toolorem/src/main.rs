mod parser;

use parser::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, path, template } => {
            println!("Создаем проект: {}", name);
            println!("Путь: {:?}", path);
            println!("По шаблону: {:?}", template)
        },
    }
}
