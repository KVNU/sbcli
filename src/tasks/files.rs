use std::{collections::HashMap, fs, hash::Hash, path::Path};

use convert_case::{Case, Casing};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::{meta::Meta, Config};

use super::{
    get::{get_progress, get_tasks},
    Task,
};

/// Creates the exercises directory if it doesn't exist
pub fn init_filesystem() -> anyhow::Result<()> {
    let cfg = Config::load()?;
    let path = Path::new(&cfg.exercises_dir);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    if !cfg.meta_path.exists() {
        let progress = Meta::new(Vec::new());
        let progress_json = serde_json::to_string(&progress)?;
        fs::write(&cfg.meta_path, progress_json)?;
    }

    Ok(())
}

/// Creates the progress file if it doesn't exist
/// and initializes it with the number of tasks for the course and the order
pub fn init_progress() -> anyhow::Result<()> {
    // let cfg = Config::load()?;
    let progress = Meta::load()?;
    if progress.total_tasks == 0 {
        let tasks = get_tasks()?;
        let progress = Meta::new(tasks);
        progress.save()?;
    }

    Ok(())
}

/// Manages tracking of progress
/// - Updates the progress files list of solved tasks
/// - Updates the next task to be solved according to the orderings of the tasks
pub fn update_progress_file() -> anyhow::Result<()> {
    let solved = get_progress()?;

    let mut progress = Meta::load()?;

    progress.set_solved_tasks_ids(solved);
    progress.save()?;

    Ok(())
}

/// Reads the content of an exercise and its metadata
pub fn read_task_and_id(path: &Path) -> anyhow::Result<(String, usize)> {
    let content = fs::read_to_string(path)?;
    let task_id = extract_task_id_from_directory(path)?;

    Ok((content, task_id))
}

/// Replicates the directory structure of the exercises on the server
/// in the exercises directory
pub fn sync_exercises() -> anyhow::Result<()> {
    let cfg = Config::load().expect("Unable to load config");

    init_filesystem().expect("Unable to init filesystem"); // TODO maybe this should be done elsewhere
    init_progress().expect("Unable to init progress");

    let tasks = get_tasks().expect("Unable to get tasks");

    // create directory structure
    for task in tasks {
        let path = Path::new(&cfg.exercises_dir).join(format!(
            "{}_{}",
            task.taskid,
            &task.task_description.shortname.to_case(Case::Snake)
        ));
        if !path.exists() {
            fs::create_dir(&path)?;
        }

        let readme_file_path = path.join("README.md");
        let file_name = format!("{}.{}", task.taskid, task.lang);
        let file_path = path.join(file_name);

        if !file_path.exists() {
            fs::write(file_path, task.task_description.default_editor_input)?;
            fs::write(readme_file_path, task.task_description.task)?;
            // fs::write(meta_file_path, task.taskid.to_string())?;
        }
    }

    update_progress_file()?;

    Ok(())
}

fn extract_task_id_from_directory(path: &Path) -> anyhow::Result<usize> {
    let directory_name = path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|file_name| file_name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to extract directory name from path"))?;

    let re = Regex::new(r"^(\d+)_")?;
    let task_id = re
        .captures(directory_name)
        .and_then(|cap| cap.get(1).map(|m| m.as_str()))
        .ok_or_else(|| anyhow::anyhow!("Failed to extract task ID from directory name"))?;

    let task_id = task_id.parse::<usize>()?;
    Ok(task_id)
}
