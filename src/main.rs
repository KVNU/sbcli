mod config;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Dbg {
        #[arg(short, long)]
        print_cli: bool,
    },
    /// Downloads an exercise and saves it in the exercise directory
    Download {
        #[arg(short, long)]
        url: String,
    },
    Submit {
        path: PathBuf,
    },
    Test {
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Dbg { print_cli }) => {
            if *print_cli {
                dbg!(cli);
            }
        }

        None => {}

        _ => {}
    }
}
