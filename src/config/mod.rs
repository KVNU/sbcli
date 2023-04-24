pub mod meta;
mod settings;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = "sbcli";
pub const CONFIG_NAME: &str = "config";
pub const META_FILE_NAME: &str = "meta";
pub const DIRECTORY_DIR_NAME: &str = "tasks";
// pub const SESSION_DURATION_SECONDS: usize = 60 * 60 * 24 * 7 * 4; // 1 month
pub const SESSION_DURATION_SECONDS: usize = 60 * 60 * 12; // 12 hours

/// Application configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub host: String,
    pub user: String,
    pub course: String,
    pub token: String,
    pub last_login_time: Option<DateTime<Utc>>,
    // pub settings: settings::Settings,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            host: "".to_string(),
            user: "".to_string(),
            course: "".to_string(),
            token: "".to_string(),
            last_login_time: None,
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

    pub fn store(&self) -> Result<(), confy::ConfyError> {
        confy::store(APP_NAME, CONFIG_NAME, self)
    }

    pub fn is_token_valid(&self) -> bool {
        if let Some(last_login_time) = self.last_login_time {
            let now = Utc::now();
            let duration = now.signed_duration_since(last_login_time);
            let seconds = duration.num_seconds() as usize;
            seconds < SESSION_DURATION_SECONDS
        } else {
            false
        }
    }

    // pub fn show() -> Result<(), confy::ConfyError> {
    //     let cfg = Self::load()?;
    //     println!("{:#?}", cfg);
    //     Ok(())
    // }
}
