use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod files;
pub mod get;
pub mod submit;

/// Represents an exercise
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Task {
    pub taskid: usize,
    #[serde(rename = "taskDescription")]
    pub task_description: TaskDescription,
    pub lang: String,
    pub tags: Vec<Tag>,
    #[serde(rename = "orderBy")]
    pub order_by: usize,
    pub prerequisites: Vec<String>,
    #[serde(rename = "unlockableAssets")]
    pub unlockable_assets: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TaskDescription {
    pub task: String,
    pub title: String,
    pub shortname: String,
    #[serde(rename = "defaultEditorInput")]
    pub default_editor_input: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub points: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Submission {
    pub user: String,
    pub course: String,
    pub taskid: usize,
    pub timestamp: usize,
    pub content: String, // figure out how to deserialize this
    #[serde(rename = "resultType")]
    pub result_type: String,
    pub simplified: Simplified,
    pub details: HashMap<String, String>,
    pub score: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Simplified {
    pub compiler: Compiler,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Compiler {
    pub stdout: String,
    #[serde(rename = "exitCode")]
    pub exit_code: isize,
}
