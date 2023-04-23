use std::{
    collections::HashMap,
    fs,
    hash::Hash,
    path::{Path, PathBuf},
};

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
        let meta = Meta::default();
        let meta_json = serde_json::to_string(&meta)?;
        fs::write(&cfg.meta_path, meta_json)?;
    }

    Ok(())
}

/// Creates the meta file if it doesn't exist
/// and initializes it with the number of tasks for the course and the order
pub fn init_meta(tasks: &Vec<Task>) -> anyhow::Result<()> {
    // let cfg = Config::load()?;
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
pub fn update_meta() -> anyhow::Result<()> {
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

/// Generates a path to a task directory
/// The format is: <task_order>_<task_shortname>
pub fn make_task_path(task: &Task) -> anyhow::Result<PathBuf> {
    let cfg = Config::load()?;
    let dir_path =
        format!("{:04}_{}", task.order_by, task.task_description.shortname).to_case(Case::Snake);
    let path = Path::new(&cfg.exercises_dir)
        .join(dir_path)
        .join(format!("{}.{}", task.taskid, task.lang));

    Ok(path)
}

// pub fn make_task_path(task: &Task) -> anyhow::Result<PathBuf> {
//     let cfg = Config::load()?;
//     let path = Path::new(&cfg.exercises_dir).join(format!(
//         "{}_{}",
//         task.taskid,
//         task.task_description.shortname.to_case(Case::Snake)
//     ));

//     if !path.exists() {
//         // fs::create_dir_all(&path)?;
//         fs::create_dir(&path)?;
//     }

//     let file_name = format!("{}.{}", task.taskid, task.lang);
//     let file_path = path.join(file_name);

//     Ok(file_path)
// }

/// Replicates the directory structure of the exercises on the server
/// in the exercises directory
pub fn sync_exercises() -> anyhow::Result<()> {
    init_filesystem().expect("Unable to init filesystem"); // TODO maybe this should be done elsewhere

    let tasks = get_tasks().expect("Unable to get tasks");

    init_meta(&tasks).expect("Unable to init progress");

    // create directory structure
    for task in tasks {
        let task_path = make_task_path(&task)?;
        let parent_dir = task_path.parent().unwrap();

        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }

        let readme_file_path = parent_dir.join("README.md");

        if !task_path.exists() {
            let content = if task.task_description.default_editor_input.is_empty() {
                "// Write your code here, and submit your solution once you're done!\n// Read the README for instructions\n".into()
            } else {
                task.task_description.default_editor_input
            };

            fs::write(task_path, content)?;
            fs::write(readme_file_path, task.task_description.task)?;
        }
    }

    update_meta()?;

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
