pub mod meta;
mod settings;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const APP_NAME: &str = "sbcli";
const CONFIG_NAME: &str = "config";
const META_FILE_NAME: &str = ".meta.json";

/// Application configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub host: String,
    pub user: String,
    pub course: String,
    pub token: Option<String>,
    pub exercises_dir: std::path::PathBuf,
    pub meta_path: std::path::PathBuf,
    // pub settings: settings::Settings,
}

impl Default for Config {
    fn default() -> Self {
        // on windows, the default exercises directory is %USERPROFILE%\sbcli
        // on linux, the default exercises directory is $HOME/sbcli
        // let exercises_dir = dirs::home_dir()
        //     .map(|mut p| {
        //         p.push(APP_NAME);
        //         p
        //     })
        //     .unwrap_or_else(|| PathBuf::from(APP_NAME));
        let exercises_dir = PathBuf::from("./dbg/exercises");
        let progress_path = exercises_dir.join(META_FILE_NAME);

        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            host: "".to_string(),
            user: "".to_string(),
            course: "".to_string(),
            token: None,
            exercises_dir,
            meta_path: progress_path, // settings: settings::Settings::default(),
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
