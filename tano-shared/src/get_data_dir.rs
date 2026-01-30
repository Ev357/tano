use std::{env, path::PathBuf};

use color_eyre::eyre::{Result, eyre};

use crate::project_directory::project_directory;

pub fn get_data_dir() -> Result<PathBuf> {
    if let Ok(path) = env::var("TANO_DATA") {
        return Ok(PathBuf::from(path));
    } else if let Some(proj_dirs) = project_directory() {
        return Ok(proj_dirs.data_local_dir().to_path_buf());
    }

    Err(eyre!("Unable to find data directory for tano"))
}
