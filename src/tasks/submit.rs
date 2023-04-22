use std::{collections::HashMap, fs, path::PathBuf};

use reqwest::header::{AUTHORIZATION, COOKIE};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Deserialize)]
struct SubmissionResult {
    user: String,
    course: String,
    taskid: usize,
    timestamp: usize,
    content: String, // figure out how to deserialize this
    #[serde(rename = "resultType")]
    result_type: String,
    simplified: Simplified,
    details: HashMap<String, String>,
    score: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Simplified {
    compiler: Compiler,
}

#[derive(Debug, Deserialize, Serialize)]
struct Compiler {
    stdout: String,
    exitCode: i32,
}

#[derive(Debug, Deserialize)]
struct SubmissionResponse {
    result: SubmissionResult,
    #[serde(skip)]
    #[serde(rename = "newUnlockedAssets")]
    new_unlocked_assets: Vec<String>, // don't know the structure of this object, if it's not just a string
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
        .post(url)
        .json(&request_body)
        // .header(
        //     AUTHORIZATION,
        //     format!("Bearer: {}", cfg.token.clone().unwrap()),
        // )
        .header(COOKIE, format!("token={}", cfg.token.unwrap()))
        .send()?;

    if res.status().is_success() {
        let res: SubmissionResponse = res.json()?;
        dbg!(&res);

        if res.result.score >= 1 {
            println!("Task solved successfully!");
        } else {
            println!("Task not solved successfully.");
        }
    } else {
        dbg!(res);
        return Err(anyhow::anyhow!("Response indicates failure"));
    }

    Ok(())
}
