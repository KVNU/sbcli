use std::{
    fs,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};

use crate::config::{meta::Meta, Config};

use super::{
    get::{get_progress, get_submissions, get_tasks},
    Task,
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
pub fn make_task_path(task: &Task) -> anyhow::Result<(PathBuf, PathBuf)> {
    let meta = Meta::load()?;
    let dir_path = Path::new(&meta.directory_dir()).join(
        format!("{:04}_{}", task.order_by, task.task_description.shortname).to_case(Case::Snake),
    );

    let task_path = dir_path.join(format!("{}.{}", task.taskid, task.lang));

    Ok((dir_path, task_path))
}

/// Replicates the directory structure of the exercises on the server
/// in the exercises directory
/// TODO fix force logic
pub fn sync_exercises(force: bool, submissions: bool) -> anyhow::Result<()> {
    init_filesystem()?;
    let tasks = get_tasks()?;

    for task in &tasks {
        create_task_directories(task)?;
        if submissions {
            create_submissions_directory(task)?;
            save_submissions(task)?;
        }
        write_task_files(task, force)?;
    }

    // HACK positional stuff. make this more robust
    init_meta(&tasks)?;
    update_meta_progress()?;
    Ok(())
}

fn create_task_directories(task: &Task) -> anyhow::Result<()> {
    let (dir_path, _) = make_task_path(task)?;
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }
    Ok(())
}

fn create_submissions_directory(task: &Task) -> anyhow::Result<()> {
    let (dir_path, _) = make_task_path(task)?;
    let submissions_dir = dir_path.join("submissions");
    if !submissions_dir.exists() {
        fs::create_dir(submissions_dir)?;
    }
    Ok(())
}

fn save_submissions(task: &Task) -> anyhow::Result<()> {
    let submissions = get_submissions(task.taskid)?;
    let (dir_path, _) = make_task_path(task)?;
    let submissions_dir = dir_path.join("submissions");

    for submission in submissions {
        let path = submissions_dir.join(format!("{}.{}", submission.timestamp, task.lang));
        let metadata_path = submissions_dir.join(format!(
            "{}.{}.metadata.json",
            submission.timestamp, task.lang
        ));
        if !path.exists() {
            fs::write(path, &submission.content)?;
            fs::write(
                metadata_path,
                serde_json::to_string_pretty(&submission.compiler_msg()?)?,
            )?;
        }
    }
    Ok(())
}

fn write_task_files(task: &Task, force: bool) -> anyhow::Result<()> {
    let (dir_path, task_path) = make_task_path(task)?;
    let readme_file_path = dir_path.join("README.md");

    if force || !task_path.exists() {
        let content = if task.task_description.default_editor_input.is_empty() {
            "// Write your code here, and submit your solution once you're done!\n// Read the README for instructions\n"
        } else {
            &task.task_description.default_editor_input
        };

        fs::write(task_path, content)?;
        fs::write(readme_file_path, &task.task_description.task)?;
    }
    Ok(())
}
