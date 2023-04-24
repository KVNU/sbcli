use reqwest::header::COOKIE;

use crate::config::Config;

use super::models::{SubmissionGet, Task};

/// GET /api/courses/{courseId}/tasks
/// Needs to be authenticated
pub fn get_tasks() -> anyhow::Result<Vec<Task>> {
    let cfg = Config::load()?;
    let url = format!("{}/api/courses/{}/tasks", cfg.host, cfg.course);

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token))
        .send()?;
    let tasks: Vec<Task> = res.json()?;
    Ok(tasks)
}

/// GET /api/courses/{courseId}/tasks/{taskId}/submissions
/// Needs to be authenticated
/// Returns a list of submissions for the given task
pub fn get_submissions(task_id: usize) -> anyhow::Result<Vec<SubmissionGet>> {
    let cfg = Config::load()?;
    let url = format!(
        "{}/api/courses/{}/tasks/{}/submissions",
        cfg.host, cfg.course, task_id
    );

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token))
        .send()?;

    Ok(res.json()?)
}

/// GET /api/courses/{courseId}/tasks/{taskId}/submissions/{submissionId}
/// This endpoint returns a submission with all the details, like test cases
pub fn get_submission(task_id: usize, submission_id: usize) -> anyhow::Result<serde_json::Value> {
    let cfg = Config::load()?;
    let url = format!(
        "{}/api/courses/{}/tasks/{}/submissions/{}",
        cfg.host, cfg.course, task_id, submission_id
    );

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token))
        .send()?;

    Ok(res.json::<serde_json::Value>()?)
}

/// This is a very expensive operation. It gets all the submissions for a task and then gets the details for each submission.
pub fn get_detailed_submissions(task_id: usize) -> anyhow::Result<Vec<serde_json::Value>> {
    let submissions = get_submissions(task_id)?;
    dbg!(submissions.len());
    let mut detailed_submissions = Vec::new();
    for submission in submissions {
        dbg!(submission.id);
        let detailed_submission = get_submission(task_id, submission.id)?;
        dbg!(&detailed_submission);
        detailed_submissions.push(detailed_submission);
    }
    Ok(detailed_submissions)
}

/// GET /api/courses/{courseId}/progress
/// Needs to be authenticated
/// Returns a list of task ids that have been solved
pub fn get_progress() -> anyhow::Result<Vec<usize>> {
    let cfg = Config::load()?;
    let url = format!("{}/api/courses/{}/progress", cfg.host, cfg.course);

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(COOKIE, format!("token={}", cfg.token))
        .send()?;
    let progress: Vec<usize> = res.json()?;
    Ok(progress)
}
