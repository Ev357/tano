use std::{env, path::PathBuf};

use color_eyre::eyre::{Result, eyre};
use directories::BaseDirs;

pub fn get_config_dir() -> Result<PathBuf> {
    if let Ok(path) = env::var("TANO_CONFIG") {
        return Ok(PathBuf::from(path));
    }

    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(path).join("tano"));
    }

    if let Some(base_dirs) = BaseDirs::new() {
        return Ok(base_dirs.home_dir().join(".config").join("tano"));
    }

    Err(eyre!("Unable to find config directory"))
}
