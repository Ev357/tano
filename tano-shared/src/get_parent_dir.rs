use std::path::Path;

use color_eyre::eyre::{Result, eyre};

pub fn get_parent_dir(path: &Path) -> Result<&Path> {
    let parent = path
        .parent()
        .ok_or(eyre!("Unable to get parent directory of: {:?}", path))?;

    Ok(parent)
}
