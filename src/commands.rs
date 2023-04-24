use std::path::Path;

use anyhow::Ok;
use colored::Colorize;
use itertools::Itertools;
use serde_json::json;

use crate::{
    auth::{self, ensure_auth},
    config::{self, Config},
    requests,
    tasks::{self, files::sync_tasks_async},
    util::prompt_for_consent,
};

pub async fn configure(username: &str, course: &str, host: &str) -> anyhow::Result<()> {
    let mut cfg = Config::load()?;

    cfg.version = env!("CARGO_PKG_VERSION").to_string();
    cfg.course = course.to_string();
    cfg.user = username.to_string();
    cfg.host = host.to_string();

    Config::store(&cfg)?;

    if prompt_for_consent("Do you want to sync the exercises now?") {
        ensure_auth().await?;
        sync(false, true).await?;

        println!("{}", "Setup complete!".green());
    } else {
        let command_str = format!("`{} sync`", env!("CARGO_PKG_NAME")).on_bright_black();
        let msg = format!("Configuration complete!\nYou'll need to run {} to sync the exercises before you can start working on them.", command_str);
        println!("{}", msg);
    };

    Ok(())
}

pub async fn login() -> anyhow::Result<()> {
    ensure_configured()?;

    auth::login().await
}

pub async fn sync(force: bool, submissions: bool) -> anyhow::Result<()> {
    ensure_configured_and_auth().await?;
    let api_client = requests::ApiClient::new()?;

    // sync_exercises(force, submissions)?;
    sync_tasks_async(force, submissions, &api_client).await?;
    let meta = config::meta::Meta::load()?;

    let command_str = format!("{} start", env!("CARGO_PKG_NAME")).on_bright_black();
    println!(
        "Synced exercises! You can find them in {} or use `{}` to start working on them in your editor.",
        meta.directory_dir().display().to_string().bright_blue(),
        command_str.on_black(),
    );

    Ok(())
}

pub async fn submit_task(path: &Path) -> anyhow::Result<()> {
    ensure_fully_setup().await?;

    let client = requests::ApiClient::new()?;
    let res = client.submit_task(path).await?;

    // check if the task was solved correctly
    if res.result.was_successful() {
        println!(
            "{}",
            "Congratulations! You solved the task correctly!".green()
        );

        let mut meta = config::meta::Meta::load()?;
        meta.add_solved_task_id(res.result.taskid);
        meta.save()?;
    } else {
        println!("Unfortunately, your solution is not correct yet. Keep trying!\n");
        // :')
        // what a mess of code. I'm sorry.
        // println!("{}", "Here's a cookie for your effort: 🍪".bright_blue());
        if !res.result.simplified.compiler.stdout.is_empty() {
            println!("Compiler Output:\n");
            println!("{}", res.result.simplified.compiler.stdout.on_black());
        }

        let submissions = client.get_detailed_submissions(res.result.taskid).await?;
        // find latest submission
        // Object {
        //     "content": String(""),
        //     "course": String(""),
        //     "details": Object {},
        //     "id": Number(0),
        //     "resultType": String("WRONG_ANSWER|COMPILER_ERROR|SUCCESS"),
        //     "score": Number(0),
        //     "simplified": Object {
        //         "compiler": Object {
        //             "exitCode": Number(0),
        //             "stdout": String(""),
        //         },
        //         "testCase": Object {
        //             "exitCode": Number(0),
        //             "expectedStdout": String("n"),
        //             "stdin": String(""),
        //             "stdout": String(""),
        //         },
        //     },
        //     "taskid": Number(0),
        //     "timestamp": String(""),
        // }
        if let Some(latest_submission) = submissions.iter().max_by_key(|s| {
            let timestamp = s.get("timestamp").unwrap();

            if timestamp.is_u64() {
                timestamp.as_u64().unwrap()
            } else {
                timestamp.as_str().unwrap().parse::<u64>().unwrap()
            }
        }) {
            if let Some(object) = latest_submission.get("simplified") {
                if let Some(object) = object.get("testCase") {
                    println!(
                        "Tests:\n{}\n",
                        object
                            .get("message")
                            .unwrap_or(&json!("()"))
                            .as_str()
                            .unwrap()
                    );
                    if let Some(expected_stdout) = object.get("expectedStdout") {
                        println!(
                            "Expected stdout:\n{}",
                            expected_stdout.as_str().unwrap_or("")
                        );
                    }
                    if let Some(stdout) = object.get("stdout") {
                        println!("Actual stdout:\n{}", stdout.as_str().unwrap_or(""));
                    }
                }
            }
        }

        // Print result type and exit code
        println!(
            "{} | Exit Code: {}\n",
            res.result.result_type.bright_red(),
            res.result.simplified.compiler.exit_code
        );
    }
    Ok(())
}

pub async fn list_tasks() -> anyhow::Result<()> {
    ensure_fully_setup().await?;

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

    // total solved
    let ratio = solved.len() as f32 / tasks.len() as f32;

    println!(
        "You have solved {} out of {} tasks ({}%)",
        solved.len().to_string().bright_green(),
        tasks.len(),
        (ratio * 100.0).round().to_string().bright_blue()
    );

    Ok(())
}

pub async fn start_task(task_id: Option<usize>) -> anyhow::Result<()> {
    ensure_fully_setup().await?;
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

        anyhow::bail!("{}", "Please configure the CLI first.".bright_red());
    }

    Ok(())
}

/// Ensure that Meta has been set up, this is the case after a sync
fn ensure_tasks_init() -> anyhow::Result<()> {
    let meta = config::meta::Meta::load()?;

    if meta.tasks().is_empty() {
        anyhow::bail!("{}", "Please sync the exercises first.".bright_red());
    }

    Ok(())
}

async fn ensure_configured_and_auth() -> anyhow::Result<()> {
    ensure_configured()?;
    ensure_auth().await?;

    Ok(())
}

async fn ensure_fully_setup() -> anyhow::Result<()> {
    ensure_configured()?;
    ensure_tasks_init()?;
    ensure_auth().await?;

    Ok(())
}
