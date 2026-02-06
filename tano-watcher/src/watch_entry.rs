use std::path::PathBuf;

use crate::watch_type::WatchType;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WatchEntry {
    pub path: PathBuf,
    pub watch_type: WatchType,
}
