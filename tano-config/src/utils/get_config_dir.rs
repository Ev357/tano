use std::path::PathBuf;

use color_eyre::eyre::{Result, eyre};

use crate::utils::project_directory::project_directory;

pub fn get_config_dir() -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("TANO_CONFIG") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(eyre!("Unable to find config directory for tano",));
    };
    Ok(directory)
}
