use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::config::Config;

use super::Task;

/// Reads the metadata of an exercise located at `path`
/// The metadata is stored in a .meta file in the same directory as the exercise,
/// or in a comment at the top of the exercise file "task: <task_id>"
fn read_exercise_with_metadata(path: PathBuf) -> anyhow::Result<()> {
    todo!()
}

pub fn get_tasks() -> anyhow::Result<Vec<Task>> {
    let cfg = Config::load()?;
    let url = format!("{}/api/courses/{}/tasks", cfg.host, cfg.course);
    dbg!(&url);

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("Cookie", format!("token={}", cfg.token.unwrap()))
        .send()?;
    let tasks: Vec<Task> = res.json()?;
    Ok(tasks)
}
