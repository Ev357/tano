use color_eyre::eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use crate::{watch_entry::WatchEntry, watch_event::WatchEvent, watch_id::WatchId};

pub trait Watcher {
    fn new(event_handler: UnboundedSender<Result<WatchEvent>>) -> Result<Self>
    where
        Self: Sized;
    fn watch(&mut self, watch_entry: WatchEntry) -> Result<()>;
    fn unwatch(&mut self, watch_id: &WatchId) -> Result<()>;
}
