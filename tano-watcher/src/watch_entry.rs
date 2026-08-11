use std::path::PathBuf;

use crate::{watch_filter::WatchFilter, watch_id::WatchId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WatchEntry {
    pub id: WatchId,
    pub path: PathBuf,
    pub filter: WatchFilter,
}
