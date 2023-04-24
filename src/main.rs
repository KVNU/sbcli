mod auth;
mod commands;
mod config;
mod requests;
mod tasks;
mod util;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use commands::{list_tasks, start_task, submit_task, sync};
use config::Config;

use crate::commands::configure;
use crate::commands::login;
use crate::requests::ApiClient;

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
    // #[cfg(debug_assertions)]
    Dbg,
    /// Configure the CLI
    Configure {
        #[arg(short, long)]
        username: String,
        /// The course name, e.g. "ckurs"
        #[arg(short, long)]
        course: String,
        /// The host of the SmartBeans instance, e.g. https://c109-223.cloud.gwdg.de
        #[arg(long)]
        host: String,
    },
    // TODO: 1. token login 2. select course from list of options
    /// Login to SmartBeans
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
    #[cfg(debug_assertions)] // TODO: implement
    /// Show your progress
    Progress,
    /// Work on the next task, or the task with the given ID
    Start {
        task_id: Option<usize>,
    },
    /// Submit an exercise to SmartBeans
    Submit {
        path: PathBuf,
    },
    /// Run the tests for a local exercise
    Test {
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Using custom config
    if let Some(config_path) = cli.config {
        let config: Config = confy::load_path(config_path)?;
        config.store()?;
    }

    match &cli.command {
        // #[cfg(debug_assertions)]
        Some(Commands::Dbg) => {
            let start = std::time::Instant::now();

            let api_client = ApiClient::new()?;
            let tasks = api_client.get_tasks().await?;

            let mut task_futures = Vec::new();
            for task in tasks {
                println!("task: {}, {}", task.taskid, task.task_description.shortname);
                let future = api_client.get_detailed_submissions(task.taskid);
                task_futures.push(future);
            }

            let all_tasks = futures::future::join_all(task_futures).await;
            let all_tasks = all_tasks.into_iter().flatten().collect::<Vec<_>>();

            // for task in all_tasks {
            //     println!("task: {}, {}", task.taskid, task.task_description.shortname);
            //     let _ = requests::get_detailed_submissions(task.taskid, &client).await?;
            // }

            let elapsed = start.elapsed();
            dbg!(all_tasks.len());
            dbg!(elapsed);
        }

        Some(Commands::Configure {
            username,
            course,
            host,
        }) => {
            configure(username, course, host).await?;
        }

        Some(Commands::List) => {
            dbg!("list tasks");
            list_tasks()?;
        }

        Some(Commands::Login) => {
            login()?;
        }

        Some(Commands::Start { task_id }) => {
            start_task(*task_id)?;
        }

        Some(Commands::Submit { path }) => {
            submit_task(path).await?;
        }

        Some(Commands::Sync { force, submissions }) => {
            sync(*force, *submissions).await?;
        }

        _ => {}
    };

    Ok(())
}
