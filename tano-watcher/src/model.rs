use std::collections::HashSet;

use crate::watch_entry::WatchEntry;

pub trait WatcherModel: Send + Sync + 'static {
    fn entries(&self) -> HashSet<WatchEntry>;
}
