mod auth;
mod commands;
mod config;
mod tasks;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use commands::{list_tasks, start_task, submit_task, sync};
use config::Config;

use crate::commands::configure;
use crate::commands::login;

#[derive(Debug, Parser)]
struct Cli {
    /// Use a custom config
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
    /// Configure the CLI
    Configure {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        course: String,
        /// The base URL of the SmartBeans instance, e.g. https://smartbeans.de
        #[arg(long)]
        host: String,
    },
    /// TODO: 1. token login 2. select course from list of options
    Login,
    /// Get the tasks for the current course and save them locally
    Sync {
        /// Force sync even if the exercises directory is not empty
        /// This will overwrite any local exercises with the latest submission on SmartBeans
        #[arg(short, long)]
        force: bool,
        /// Get all submissions for each task
        #[arg(short, long)]
        submissions: bool,
    },
    /// List all tasks and their current status
    List,
    /// Show your progress
    Progress,
    /// Work on the next task, or the task with the given ID
    Start { task_id: Option<usize> },
    /// Submit an exercise to SmartBeans
    Submit { path: PathBuf },
    /// Run the tests for a local exercise
    Test { path: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Using custom config
    if cli.config.is_some() {
        Config::store(&Config::default())?;
        return Ok(());
    }

    match &cli.command {
        Some(Commands::Configure {
            username,
            course,
            host,
        }) => {
            configure(username, course, host)?;
        }

        Some(Commands::Dbg { print_cli: _ }) => {
            // sync()?;
        }

        Some(Commands::List) => {
            list_tasks()?;
        }

        Some(Commands::Login) => {
            login()?;
        }

        Some(Commands::Start { task_id }) => {
            start_task(*task_id)?;
        }

        Some(Commands::Submit { path }) => {
            submit_task(path)?;
        }

        Some(Commands::Sync { force, submissions }) => {
            sync(*force, *submissions)?;
        }

        _ => {}
    };

    Ok(())
}
