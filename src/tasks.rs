use std::{collections::HashMap, fs, path::PathBuf};

use reqwest::header::{AUTHORIZATION, COOKIE};
use serde::Deserialize;

use crate::config::Config;

#[derive(Debug, Default, Deserialize)]
struct SubmissionResult {
    user: String,
    course: String,
    taskid: String,
    timestamp: String,
    content: String,
    #[serde(rename = "resultType")]
    result_type: String,
    simplified: String,
    details: String,
    score: usize,
}

#[derive(Debug, Default, Deserialize)]
struct SubmissionResponse {
    result: SubmissionResult,
    #[serde(skip)]
    #[serde(rename = "newUnlockedAssets")]
    new_unlocked_assets: Option<String>,
}

pub fn submit_task(task_id: isize, path: PathBuf) -> anyhow::Result<()> {
    let cfg: Config = Config::load()?;
    let url = format!(
        "{}/api/courses/{}/tasks/{}/submissions",
        cfg.host, cfg.course, task_id
    );
    // read task content from path as string
    let contents = fs::read_to_string(path)?;

    let client = reqwest::blocking::Client::new();

    let mut request_body = HashMap::new();
    request_body.insert("submission", &contents);
    let res = client
        .post(&url)
        .json(&request_body)
        .header(AUTHORIZATION, format!("Bearer: {}", cfg.token.clone().unwrap()))
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;

    if res.status().is_success() {
        dbg!(&res.status());

        let res: SubmissionResponse = res.json()?;
        dbg!(&res);
    } else {
        return Err(anyhow::anyhow!("Response indicates failure"));
    }

    Ok(())
}
