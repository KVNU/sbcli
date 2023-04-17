mod settings;

use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub exercises_dir: std::path::PathBuf,
    pub settings: settings::Settings,
}
