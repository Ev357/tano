use std::path::PathBuf;

use crate::watch_id::WatchId;

#[derive(Debug)]
pub enum WatcherMsg {
    WatchEvent { path: PathBuf, watch_id: WatchId },
}
