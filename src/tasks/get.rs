use reqwest::header::COOKIE;

use crate::config::Config;

use super::{Submission, Task};

pub fn get_tasks() -> anyhow::Result<Vec<Task>> {
    let cfg = Config::load()?;
    let url = format!("{}/api/courses/{}/tasks", cfg.host, cfg.course);

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;
    let tasks: Vec<Task> = res.json()?;
    Ok(tasks)
}

pub fn get_submissions(task_id: usize) -> anyhow::Result<Vec<Submission>> {
    let cfg = Config::load()?;
    let url = format!(
        "{}/api/courses/{}/tasks/{}/submissions",
        cfg.host, cfg.course, task_id
    );

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;

    let submissions: Vec<Submission> = res.json()?;
    Ok(submissions)
}
