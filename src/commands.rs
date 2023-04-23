use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::Ok;
use colored::Colorize;
use itertools::Itertools;

use crate::{
    auth::{self, ensure_auth},
    config::{self, Config},
    tasks::{self, files::sync_exercises},
};

pub fn configure(username: &str, course: &str, host: &str) -> anyhow::Result<()> {
    let mut cfg = Config::load()?;

    cfg.version = env!("CARGO_PKG_VERSION").to_string();
    cfg.course = course.to_string();
    cfg.user = username.to_string();
    cfg.host = host.to_string();

    Config::store(&cfg)?;

    if prompt_for_consent("Do you want to sync the exercises now?") {
        login()?;
        sync(false, true)?;

        println!("Setup complete!");
    } else {
        let command_str = format!("`{} sync`", env!("CARGO_PKG_NAME")).on_bright_black();
        let msg = format!("Configuration complete!\nYou'll need to run {} to sync the exercises before you can start working on them.", command_str);
        println!("{}", msg);
    };

    Ok(())
}

pub fn login() -> anyhow::Result<()> {
    ensure_configured()?;

    auth::login()
}

pub fn sync(force: bool, submissions: bool) -> anyhow::Result<()> {
    ensure_configured_and_auth()?;

    sync_exercises(force, submissions)?;
    let meta = config::meta::Meta::load()?;

    let command_str = format!("{} start", env!("CARGO_PKG_NAME")).on_bright_black();
    println!(
        "Synced exercises! You can find them in {} or use `{}` to start working on them in your editor.",
        meta.directory_dir().display().to_string().bright_blue(),
        command_str.on_black(),
    );

    Ok(())
}

pub fn submit_task(path: &PathBuf) -> anyhow::Result<()> {
    ensure_fully_setup()?;

    tasks::submit::submit(path)
}

pub fn list_tasks() -> anyhow::Result<()> {
    ensure_fully_setup()?;

    dbg!("HERE");

    let meta = config::meta::Meta::load().unwrap();
    let solved = meta.solved_task_ids();
    let tasks = meta.tasks();

    for task in tasks.iter().sorted_by(|a, b| a.order_by.cmp(&b.order_by)) {
        let status = if solved.contains(&task.taskid) {
            "Completed".green()
        } else {
            "Not yet completed".red()
        };

        let task_id_str = format!("{:5}", task.taskid).bright_blue(); // Pad task id with spaces

        println!(
            "Task ID {} | Task: {}, Status: {}",
            task_id_str,
            task.task_description.shortname.bright_blue(),
            status
        );
    }

    Ok(())
}

pub fn start_task(task_id: Option<usize>) -> anyhow::Result<()> {
    ensure_fully_setup()?;
    let meta = config::meta::Meta::load()?;

    let task_id = task_id.unwrap_or(meta.next_task_id);

    if let Some(task_path) = meta.get_task_path(task_id) {
        let mut input = String::new();
        println!("Do you want to open the task in your default editor? [Y/n]");
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "n" {
            tasks::open::open_task_in_editor(task_path)?;
        } else {
            let mut input = String::new();
            println!("Do you want to navigate to the task directory? [Y/n]");

            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() != "n" {
                let task_dir = task_path.parent().unwrap();
                let _ = open::that_in_background(task_dir);

                if input.trim().to_lowercase() != "n" {
                    let task_dir = task_path.parent().unwrap();
                    println!("To navigate to the task directory, run the following command in your terminal:");
                    println!("cd {}", task_dir.display());
                }
            }
        }
    } else {
        println!("Task with ID {} not found.", task_id);
    }

    Ok(())
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

/// Ensure that Meta has been set up, this is the case after a sync
fn ensure_tasks_init() -> anyhow::Result<()> {
    let meta = config::meta::Meta::load()?;

    if meta.tasks().is_empty() {
        anyhow::bail!("Please sync the tasks first.");
    }

    Ok(())
}

fn ensure_configured_and_auth() -> anyhow::Result<()> {
    ensure_configured()?;
    ensure_auth()?;

    Ok(())
}

fn ensure_fully_setup() -> anyhow::Result<()> {
    ensure_configured()?;
    ensure_tasks_init()?;
    // ensure_auth()?;

    Ok(())
}

// TOOD move to utils
fn prompt_for_consent(message: &str) -> bool {
    let mut input = String::new();
    print!("{} [Y/n]: ", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();
    input == "y" || input.is_empty()
}
