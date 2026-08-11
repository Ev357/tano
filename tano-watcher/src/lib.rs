use color_eyre::eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use crate::{watch_event::WatchEvent, watcher::Watcher};

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
use linux::INotifyWatcher as PlatformWatcher;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
use macos::FsEventWatcher as PlatformWatcher;

pub mod actor;
pub mod constants;
pub mod model;
pub mod watch_entry;
pub mod watch_event;
pub mod watch_filter;
pub mod watch_id;
pub mod watcher;

pub type RecommendedWatcher = PlatformWatcher;

pub fn recommended_watcher(
    event_handler: UnboundedSender<Result<WatchEvent>>,
) -> Result<RecommendedWatcher> {
    RecommendedWatcher::new(event_handler)
}
