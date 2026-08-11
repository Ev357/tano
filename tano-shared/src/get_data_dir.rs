use std::{env, path::PathBuf};

use color_eyre::eyre::{Result, eyre};
use directories::BaseDirs;

pub fn get_data_dir() -> Result<PathBuf> {
    if let Ok(path) = env::var("TANO_DATA") {
        return Ok(PathBuf::from(path));
    }

    if let Ok(path) = env::var("XDG_DATA_HOME") {
        return Ok(PathBuf::from(path).join("tano"));
    }

    if let Some(base_dirs) = BaseDirs::new() {
        return Ok(base_dirs
            .home_dir()
            .join(".local")
            .join("share")
            .join("tano"));
    }

    Err(eyre!("Unable to find data directory"))
}
