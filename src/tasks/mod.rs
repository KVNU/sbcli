use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod files;
pub mod get;
pub mod open;
pub mod submit;

/// Represents an exercise
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaskDescription {
    pub task: String,
    pub title: String,
    pub shortname: String,
    #[serde(rename = "defaultEditorInput")]
    pub default_editor_input: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub points: Option<usize>,
}

/// TODO deduplicate this by using optional fields.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubmissionPost {
    pub user: String,
    pub course: String,
    pub taskid: usize,
    pub content: String,
    #[serde(rename = "resultType")]
    pub result_type: String,
    pub simplified: Simplified,
    pub details: HashMap<String, Value>,
    pub score: f32,
}

impl SubmissionPost {
    pub fn was_successful(&self) -> bool {
        self.score >= 1.
    }
}

/// TODO deduplicate this by using optional fields.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubmissionGet {
    pub id: usize,
    pub course: String,
    pub taskid: usize,
    pub timestamp: usize,
    pub content: String, // figure out how to deserialize this
    #[serde(rename = "resultType")]
    pub result_type: String,
    simplified: String,
    #[serde(skip)]
    pub details: String,
    pub score: f32,
}

impl SubmissionGet {
    pub fn was_successful(&self) -> bool {
        self.score >= 1.
    }

    pub fn simplified(&self) -> anyhow::Result<Simplified> {
        let simplified: Simplified = serde_json::from_str(&self.simplified).unwrap();
        Ok(simplified)
    }

    pub fn compiler_msg(&self) -> anyhow::Result<Compiler> {
        Ok(self.simplified()?.compiler)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Simplified {
    pub compiler: Compiler,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Compiler {
    pub stdout: String,
    #[serde(rename = "exitCode")]
    pub exit_code: isize,
}

impl Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stdout)
    }
}
