use std::{fs, path::Path};

use convert_case::{Case, Casing};

use crate::config::Config;

use super::get::get_tasks;

/// Creates the exercises directory if it doesn't exist
pub fn init_filesystem() -> anyhow::Result<()> {
    let cfg = Config::load()?;
    let path = Path::new(&cfg.exercises_dir);
    dbg!(path);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn read_exercise_with_metadata(path: &Path) -> anyhow::Result<(String, usize)> {
    let content = fs::read_to_string(path)?;
    let meta = fs::read_to_string(path.with_extension("meta"))?;
    let task_id = meta.parse::<usize>()?;

    Ok((content, task_id))
}

/// Replicates the directory structure of the exercises on the server
/// in the exercises directory
pub fn sync_exercises() -> anyhow::Result<()> {
    let cfg = Config::load().expect("Unable to load config");

    init_filesystem().expect("Unable to init filesystem"); // TODO maybe this should be done elsewhere

    // get list of exercises from server
    let tasks = get_tasks().expect("Unable to get tasks");
    dbg!(tasks.len());
    dbg!(&tasks);

    // create directory structure
    // each exercise is in a directory named <task_id>_<task_name>
    // each exercise file is named <task_id>.<lang>
    // the default editor input is written to the file
    for task in tasks {
        let path = Path::new(&cfg.exercises_dir).join(format!(
            "{}_{}",
            task.taskid,
            &task.task_description.shortname.to_case(Case::Snake)
        ));
        if !path.exists() {
            fs::create_dir(&path).expect("Unable to create directory");
        }

        let meta_file_path = path.join(".meta");
        let file_name = format!("{}.{}", task.taskid, task.lang);
        let file_path = path.join(file_name);

        if !file_path.exists() {
            fs::write(file_path, task.task_description.default_editor_input)?;
            fs::write(meta_file_path, task.taskid.to_string())?;
        }
    }

    Ok(())
}
