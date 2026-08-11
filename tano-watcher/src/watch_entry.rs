use std::path::PathBuf;

use crate::{path_type::PathType, watch_id::WatchId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WatchEntry {
    pub id: WatchId,
    pub path: PathBuf,
    pub path_type: PathType,
}
