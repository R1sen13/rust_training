mod task;
mod manager;
mod cli;

use manager::TaskManager;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let mut manager = TaskManager::load();

    match cli.command {
        Commands::Add { title } => {
            manager.add_task(title);
            manager.save();
            println!("Task was added");
        }

        Commands::Complete { id } => {
            if manager.complete_task(id) {
                println!("Task successfully completed");
                manager.save();
            } else {
                println!("Task does not exist");
            }
        }

        Commands::List => {
            if manager.tasks().is_empty() {
                println!("No tasks found");
            } else {
                manager.print_tasks()
            }
        }

        Commands::Remove { id } => {
            if manager.remove_task(id) {
                println!("Task successfully removed");
                manager.save();
            } else {
                println!("Task does not exist");
            }
        }

    }
}
