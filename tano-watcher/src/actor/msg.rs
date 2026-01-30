use std::path::PathBuf;

use crate::watch_type::WatchType;

#[derive(Debug)]
pub enum WatcherMsg {
    FileChange {
        path: PathBuf,
        watch_type: WatchType,
    },
}
