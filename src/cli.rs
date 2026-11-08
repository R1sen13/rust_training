use clap::{ Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ttrack")]
#[command(about = "Task tracker in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
    },
    List,
    Complete {
        id: usize,
    },
    Remove {
        id: usize,
    },
}