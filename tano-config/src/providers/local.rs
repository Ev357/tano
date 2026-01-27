use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LocalConfig {
    pub path: PathBuf,
}
