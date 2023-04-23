use std::{collections::HashMap, path::Path};

use reqwest::header::COOKIE;
use serde::Deserialize;

use crate::config::{self, Config};

use super::{files::read_task_and_id, Submission};

#[derive(Debug, Deserialize)]
struct SubmissionResponse {
    result: Submission,
    #[serde(skip)]
    #[serde(rename = "newUnlockedAssets")]
    new_unlocked_assets: Vec<String>, // don't know the structure of this object, if it's not just a list of strings
}

pub fn submit(path: &Path) -> anyhow::Result<()> {
    let cfg: Config = Config::load()?;

    // TODO broken. Use Meta.directory
    let (submission_content, task_id) = read_task_and_id(&path)?;

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
        let res: SubmissionResponse = res.json()?;
        dbg!(&res);

        if res.result.score >= 1 {
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
