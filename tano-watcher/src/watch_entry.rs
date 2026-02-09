use std::path::PathBuf;

use crate::{path_type::PathType, watch_mode::WatchMode, watch_type::WatchType};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WatchEntry {
    pub path: PathBuf,
    pub path_type: PathType,
    pub watch_type: WatchType,
    pub watch_mode: WatchMode,
}
