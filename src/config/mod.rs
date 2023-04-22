mod settings;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const APP_NAME: &str = "sbcli";
const CONFIG_NAME: &str = "config";

// pub fn load<T: Deserialize<'static> + Serialize + Default + for<'de> serde::Deserialize<'de>>() -> Result<T, confy::ConfyError> {
//     confy::load::<T>(APP_NAME, CONFIG_NAME)
// }

/// Application configuration.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub host: String,
    pub user: String,
    pub course: String,
    pub token: Option<String>,
    pub exercises_dir: std::path::PathBuf,
    // pub settings: settings::Settings,
}

impl Config {
    pub fn load() -> Result<Self, confy::ConfyError> {
        confy::load::<Self>(APP_NAME, CONFIG_NAME)
    }

    pub fn store(cfg: &Self) -> Result<(), confy::ConfyError> {
        confy::store(APP_NAME, CONFIG_NAME, cfg)
    }
}
