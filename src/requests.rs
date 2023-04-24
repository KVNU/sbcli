use std::{collections::HashMap, path::Path};

use reqwest::header::COOKIE;
use reqwest::Client;
use serde::Deserialize;

use crate::{
    config::{self, Config},
    tasks::models::{SubmissionGet, SubmissionPost, Task},
};

/// An API client for SmartBeans
pub struct ApiClient {
    client: Client,
    config: Config,
}

impl ApiClient {
    pub fn new() -> anyhow::Result<Self> {
        let client = Client::new();
        let config = Config::load()?;
        Ok(Self { client, config })
    }

    pub async fn get_tasks(&self) -> anyhow::Result<Vec<Task>> {
        let url = format!(
            "{}/api/courses/{}/tasks",
            self.config.host, self.config.course
        );

        let res = self
            .client
            .get(url)
            .header(COOKIE, format!("token={}", self.config.token))
            .send()
            .await?;

        let tasks: Vec<Task> = res.json().await?;
        Ok(tasks)
    }

    pub async fn get_submission(
        host: &str,
        course: &str,
        token: &str,
        task_id: usize,
        submission_id: usize,
        client: &Client,
    ) -> anyhow::Result<serde_json::Value> {
        let url = format!(
            "{}/api/courses/{}/tasks/{}/submissions/{}",
            host, course, task_id, submission_id
        );

        let res = client
            .get(url)
            .header(COOKIE, format!("token={}", token))
            .send()
            .await?;

        Ok(res.json::<serde_json::Value>().await?)
    }

    pub async fn get_submissions(&self, task_id: usize) -> anyhow::Result<Vec<SubmissionGet>> {
        let url = format!(
            "{}/api/courses/{}/tasks/{}/submissions",
            self.config.host, self.config.course, task_id
        );

        let res = self
            .client
            .get(url)
            .header(COOKIE, format!("token={}", self.config.token))
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn get_detailed_submissions(
        &self,
        task_id: usize,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        use futures::future::join_all;

        let submissions = self.get_submissions(task_id).await?;
        let mut submission_futures = Vec::new();

        let host = &self.config.host;
        let course = &self.config.course;
        let token = &self.config.token;
        let client = &self.client;
        for submission in submissions {
            let host = host.clone();
            let course = course.clone();
            let token = token.clone();
            let client = client.clone();

            let future = tokio::spawn(async move {
                ApiClient::get_submission(&host, &course, &token, task_id, submission.id, &client)
                    .await
            });
            submission_futures.push(future);
        }

        let results = join_all(submission_futures).await;
        let mut detailed_submissions = Vec::new();

        for result in results {
            match result {
                Ok(Ok(submission)) => detailed_submissions.push(submission),
                Ok(Err(e)) => eprintln!("Error: {:?}", e),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }

        Ok(detailed_submissions)
    }

    pub async fn submit_task(&self, path: &Path) -> anyhow::Result<SubmissionResponsePost> {
        let meta = config::meta::Meta::load()?;

        let task_id = meta
            .get_task_id_from_workspace(path)
            .expect("Task not found at expected path");
        let submission_content = std::fs::read_to_string(path)?;

        let url = format!(
            "{}/api/courses/{}/tasks/{}/submissions",
            self.config.host, self.config.course, task_id
        );

        let mut request_body = HashMap::new();
        request_body.insert("submission", &submission_content);
        let res = self
            .client
            .post(url)
            .json(&request_body)
            .header(COOKIE, format!("token={}", self.config.token))
            .send()
            .await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(anyhow::anyhow!("Response indicates failure"))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SubmissionResponsePost {
    pub result: SubmissionPost,
    #[serde(skip)]
    #[serde(rename = "newUnlockedAssets")]
    pub new_unlocked_assets: Vec<String>, // don't know the structure of this object, if it's not just a list of strings
}
