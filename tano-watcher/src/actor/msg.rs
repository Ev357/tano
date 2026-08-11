use std::path::PathBuf;

use color_eyre::eyre::Report;

use crate::watch_id::WatchId;

#[derive(Debug)]
pub enum WatcherMsg {
    WatchEvent { path: PathBuf, watch_id: WatchId },
    Error(Report),
}
