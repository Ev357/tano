use std::path::PathBuf;

use color_eyre::eyre::eyre;
use serde::{Deserialize, Deserializer, de::Error};

#[derive(Debug, Deserialize, Clone)]
pub struct LocalConfig {
    #[serde(deserialize_with = "deserialize_custom_path")]
    pub path: PathBuf,
}

fn deserialize_custom_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let path_str: String = Deserialize::deserialize(deserializer)?;
    let expanded_cow = shellexpand::full(&path_str).map_err(D::Error::custom)?;

    let path = PathBuf::from(expanded_cow.as_ref());
    if !path.is_dir() {
        return Err(D::Error::custom(eyre!("Path is not valid: {path:?}")));
    }

    Ok(PathBuf::from(expanded_cow.as_ref()))
}
