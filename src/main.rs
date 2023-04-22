mod auth;
mod config;
mod tasks;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use config::Config;
use tasks::{files::sync_exercises, submit::submit_task};

use crate::auth::login;

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
        #[arg(long)]
        host: String,
    },
    /// TODO: 1. token login 2. select course from list of options
    Login,
    /// Download an exercise and save it in the exercise directory
    Download {
        #[arg(short, long)]
        url: String,
    },
    /// Get the tasks for the current course and save them locally
    Sync {
        #[arg(short, long)]
        force: bool,
    },
    /// Get the next task in order
    Next,
    /// Submit an exercise to SmartBeans
    Submit { path: PathBuf },
    /// Run the tests for a local exercise
    Test { path: PathBuf },
}

fn ensure_configured() -> anyhow::Result<()> {
    let cfg = Config::load()?;

    if cfg.user.is_empty() || cfg.course.is_empty() || cfg.host.is_empty() {
        let binary_name = std::env::args().next().unwrap();

        let output = std::process::Command::new(binary_name)
            .arg("configure")
            .arg("--help")
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            println!("{}", stdout);
        } else {
            eprintln!("{}", stderr);
        }

        anyhow::bail!("Please configure the CLI first.");
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.config.is_some() {
        Config::store(&Config::default())?;
        return Ok(());
    }

    // Config::show()?;

    match &cli.command {
        Some(Commands::Dbg { print_cli: _ }) => {
            ensure_configured()?;
            // if *print_cli {
            //     dbg!(cli);
            // }

            // get_tasks()?;
            sync_exercises()?;
        }

        Some(Commands::Configure {
            username,
            course,
            host,
        }) => {
            let mut cfg = Config::load()?;

            cfg.version = env!("CARGO_PKG_VERSION").to_string();
            cfg.course = course.to_string();
            cfg.user = username.to_string();
            cfg.host = host.to_string();

            Config::store(&cfg)?;

            // ask if we should run sync
            let mut input = String::new();
            println!("Do you want to sync the exercises now? [Y/n]");
            std::io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() != "n" {
                login()?;
                sync_exercises()?;
                print!("Synced exercises successfully.\nYou can find them in the exercises directory at {}\n", cfg.exercises_dir.display());
            }
        }

        Some(Commands::Login) => {
            ensure_configured()?;
            let _ = auth::login();
        }

        Some(Commands::Sync { force }) => {
            ensure_configured()?;
            sync_exercises()?;
        }

        Some(Commands::Next) => {
            ensure_configured()?;

            let meta = config::meta::Meta::load()?;
            let next_task_id = meta.next_task_id;
            let next_task = meta.tasks.iter().find(|t| t.taskid == next_task_id);

            if let Some(task) = next_task {
                println!(
                    "Next task({}): {}",
                    task.taskid, task.task_description.title
                );
                // ask whether to open in editor
                // let mut input = String::new();
                // println!("Do you want to open the task in your editor? [Y/n]");
                // std::io::stdin().read_line(&mut input)?;
            } else {
                println!("Couldn't determine next task.");
            }
        }

        Some(Commands::Submit { path }) => {
            ensure_configured()?;
            submit_task(path.to_path_buf())?;
        }

        _ => {}
    };

    Ok(())

    // random token with ASCII characters
    // let token = rand::thread_rng()
    //     .sample_iter(&rand::distributions::Alphanumeric)
    //     .take(30)
    //     .collect::<String>();
}
