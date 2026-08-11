use crate::watch_id::WatchId;

#[derive(Debug)]
pub struct WatchEvent {
    pub path: String,
    pub watch_id: WatchId,
}
