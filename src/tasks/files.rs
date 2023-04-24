use std::path::{Path, PathBuf};

use tokio::fs;
use tokio::io::AsyncWriteExt;

use convert_case::{Case, Casing};

use crate::{
    config::{meta::Meta, Config},
    requests::ApiClient,
};

use super::{
    get::{get_detailed_submissions, get_progress, get_submissions, get_tasks},
    models::Task,
};

/// Ensures that the configuration file exists
pub fn init_filesystem() -> anyhow::Result<()> {
    let _ = Config::load()?;
    let _ = Meta::load()?;

    Ok(())
}

/// Creates the meta file if it doesn't exist
/// and initializes it with the number of tasks for the course and the order
pub fn init_meta(tasks: &[Task]) -> anyhow::Result<()> {
    let meta = Meta::load()?;
    if meta.total_tasks == 0 {
        let meta = Meta::new(tasks);
        meta.save()?;
    }

    Ok(())
}

/// Manages tracking of progress
/// - Updates the progress files list of solved tasks
/// - Updates the next task to be solved according to the orderings of the tasks
/// TODO track progress offline
/// TODO associate with the Meta struct
pub fn update_meta_progress() -> anyhow::Result<()> {
    let solved = get_progress()?;

    let mut progress = Meta::load()?;

    progress.set_solved_tasks_ids(solved);
    progress.save()?;

    Ok(())
}

/// Generates a path to a task directory
/// The format is: <task_order>_<task_shortname>
/// Returns a tuple of the directory path (`workspace`) and the task file path
pub fn make_task_path(task: &Task, task_dir: &Path) -> anyhow::Result<(PathBuf, PathBuf)> {
    let dir_path = task_dir.join(
        format!("{:04}_{}", task.order_by, task.task_description.shortname).to_case(Case::Snake),
    );

    let task_path = dir_path.join(format!("{}.{}", task.taskid, task.lang));

    Ok((dir_path, task_path))
}

/// Replicates the directory structure of the exercises on the server
/// in the exercises directory
pub async fn sync_tasks_async(
    force: bool,
    submissions: bool,
    api_client: &ApiClient,
) -> anyhow::Result<()> {
    init_filesystem()?;
    let tasks = api_client.get_tasks().await?;

    let total_tasks = tasks.len();
    println!("Syncing {} tasks", total_tasks);
    let mut task_futures = Vec::new();
    for task in tasks.iter() {
        let future = async {
            create_task_directories(task).await?;
            if submissions {
                // NOTE: probably no big benefit if we were to use a separate futures queue for this
                sync_submissions_async(task, api_client).await?;
            }
            write_task_files(task, force).await?;

            anyhow::Ok(())
        };
        task_futures.push(future);
    }

    let _ = futures::future::join_all(task_futures).await;

    // // HACK positional stuff. make this more robust
    init_meta(&tasks)?;
    // TODO
    // this crashes with async, because it somehow creates nested runtimes ??? idk
    // Honestly, the whole persistence aspect of `Meta` should be reworked anyway
    // update_meta_progress()?;

    Ok(())
}

/// Syncs the submissions for a task
/// TODO using serde_json::Value is a bit of a hack I guess
async fn sync_submissions_async(task: &Task, api_client: &ApiClient) -> anyhow::Result<()> {
    let submissions = api_client.get_detailed_submissions(task.taskid).await?;
    let meta = Meta::load()?;
    let (dir_path, _) = make_task_path(task, meta.directory_dir())?;
    let submissions_dir = dir_path.join("submissions");

    if !submissions_dir.exists() {
        fs::create_dir(&submissions_dir).await?;
    }

    // TODO make concurrent
    for submission in submissions {
        let timestamp = submission.get("timestamp").unwrap().as_str().unwrap(); // sometimes int, sometimes string. String always deserializes correctly
        let result_type = submission.get("resultType").unwrap().as_str().unwrap();
        let path = submissions_dir.join(format!("{}-{}.{}", timestamp, result_type, task.lang));
        let metadata_path = submissions_dir.join(format!(
            "{}-{}.{}.metadata.json",
            timestamp, result_type, task.lang
        ));

        if !path.exists() {
            fs::write(path, &submission.get("content").unwrap().to_string()).await?;
            fs::write(metadata_path, serde_json::to_string_pretty(&submission)?).await?;
        }
    }
    Ok(())
}

async fn create_task_directories(task: &Task) -> anyhow::Result<()> {
    let meta = Meta::load()?;
    let (dir_path, _) = make_task_path(task, meta.directory_dir())?;
    if !dir_path.exists() {
        fs::create_dir_all(dir_path).await?;
    }
    Ok(())
}

async fn write_task_files(task: &Task, force: bool) -> anyhow::Result<()> {
    let meta = Meta::load()?;
    let (dir_path, task_path) = make_task_path(task, meta.directory_dir())?;
    let readme_file_path = dir_path.join("README.md");

    if force || !task_path.exists() {
        let content = if task.task_description.default_editor_input.is_empty() {
            "// Write your code here, and submit your solution once you're done!\n// Read the README for instructions\n"
        } else {
            &task.task_description.default_editor_input
        };

        fs::write(task_path, content).await?;
        fs::write(readme_file_path, &task.task_description.task).await?;
    }
    Ok(())
}
