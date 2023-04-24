use std::{collections::HashMap, path::Path};

use colored::Colorize;
use reqwest::header::COOKIE;
use serde::Deserialize;

use crate::config::{self, Config};

use super::models::{SubmissionGet, SubmissionPost};

#[derive(Deserialize)]
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
        .get_task_id_from_workspace(path)
        .expect("Task not found at expected path");
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
        let res: SubmissionResponsePost = dbg!(res.json()?);

        if res.result.was_successful() {
            println!("{}", "Task solved successfully!".bright_green());
            let mut meta = config::meta::Meta::load()?;
            meta.add_solved_task_id(task_id);
            meta.save()?;
        } else {
            println!("Task not solved successfully.\n-----------------------------");
            println!(
                "{} | Exit Code: {}\n",
                res.result.result_type.bright_red(),
                res.result.simplified.compiler.exit_code
            );
            println!("{}\n", res.result.simplified.compiler.stdout);
        }
    } else {
        return Err(anyhow::anyhow!("Response indicates failure"));
    }

    Ok(())
}
