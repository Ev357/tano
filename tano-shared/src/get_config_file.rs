use std::path::{Path, PathBuf};

pub fn get_config_file(config_dir: &Path) -> PathBuf {
    config_dir.join("config.toml")
}
