use std::{collections::HashMap, path::Path, task};

use reqwest::header::COOKIE;
use serde::Deserialize;

use crate::{
    config::{self, Config},
    tasks::SubmissionPost,
};

use super::SubmissionGet;

#[derive(Debug, Deserialize)]
struct SubmissionResponseGet {
    result: SubmissionGet,
    #[serde(skip)]
    #[serde(rename = "newUnlockedAssets")]
    new_unlocked_assets: Vec<String>, // don't know the structure of this object, if it's not just a list of strings
}

#[derive(Debug, Deserialize)]
struct SubmissionResponsePost {
    result: SubmissionPost,
    #[serde(skip)]
    #[serde(rename = "newUnlockedAssets")]
    new_unlocked_assets: Vec<String>, // don't know the structure of this object, if it's not just a list of strings
}

/// POST /api/courses/{courseId}/tasks/{taskId}/submissions
pub fn submit(path: &Path) -> anyhow::Result<()> {
    let cfg: Config = Config::load()?;
    let meta = config::meta::Meta::load()?;

    let task_id = meta
        .get_task_id(path)
        .expect("Task not found for current path");
    let submission_content = std::fs::read_to_string(path)?;

    let url = format!(
        "{}/api/courses/{}/tasks/{}/submissions",
        cfg.host, cfg.course, task_id
    );

    let client = reqwest::blocking::Client::new();

    let mut request_body = HashMap::new();
    request_body.insert("submission", &submission_content);
    let res = client
        .post(url)
        .json(&request_body)
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;

    if res.status().is_success() {
        dbg!(&res.text().unwrap());
        // let res: SubmissionResponse = res.json()?;
        let res: SubmissionResponsePost = SubmissionResponsePost {
            result: SubmissionPost::default(),
            new_unlocked_assets: Vec::new(),
        };
        dbg!(&res);

        if res.result.was_successful() {
            println!("Task solved successfully!");
            let mut meta = config::meta::Meta::load()?;
            meta.add_solved_task_id(task_id);
            meta.save()?;
        } else {
            println!("Task not solved successfully.");
        }
    } else {
        dbg!(res);
        return Err(anyhow::anyhow!("Response indicates failure"));
    }

    Ok(())
}
