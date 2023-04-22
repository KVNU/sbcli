use std::{fs, path::Path};

use crate::config::Config;

/// Creates the exercises directory if it doesn't exist
pub fn init_filesystem() -> anyhow::Result<()> {
    let cfg = Config::load()?;
    let path = Path::new(&cfg.exercises_dir);
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}