pub mod meta;
mod settings;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const APP_NAME: &str = "sbcli";
const CONFIG_NAME: &str = "config";

/// Application configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub host: String,
    pub user: String,
    pub course: String,
    pub token: Option<String>,
    // pub settings: settings::Settings,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            host: "".to_string(),
            user: "".to_string(),
            course: "".to_string(),
            token: None,
        }
    }
}

impl Config {
    // pub fn init () -> Result<Self, confy::ConfyError> {
    //     let cfg = Self::load()?;
    //     Ok(cfg)
    // }

    pub fn load() -> Result<Self, confy::ConfyError> {
        confy::load::<Self>(APP_NAME, CONFIG_NAME)
    }

    pub fn store(cfg: &Self) -> Result<(), confy::ConfyError> {
        confy::store(APP_NAME, CONFIG_NAME, cfg)
    }

    pub fn show() -> Result<(), confy::ConfyError> {
        let cfg = Self::load()?;
        println!("{:#?}", cfg);
        Ok(())
    }
}
