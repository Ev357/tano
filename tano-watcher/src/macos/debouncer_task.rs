use std::{collections::HashMap, path::Path};

use color_eyre::eyre::Result;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::{AbortHandle, JoinHandle, JoinSet},
    time::sleep,
};

use crate::{
    constants::DEBOUNCE_DURATION,
    macos::{
        fs_event::{FsEvent, FsEventFlag},
        watch_map::WatchMap,
    },
    watch_event::WatchEvent,
    watch_filter::WatchFilter,
    watch_id::WatchId,
};

#[derive(Debug)]
pub enum DebouncerCommand {
    Event {
        event: FsEvent,
    },
    Watch {
        id: WatchId,
        path: String,
        filter: WatchFilter,
    },
    Unwatch {
        id: WatchId,
    },
}

pub fn debouncer_task(
    mut cmd_rx: UnboundedReceiver<DebouncerCommand>,
    tx: UnboundedSender<Result<WatchEvent>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut watch_map = WatchMap::new();
        let mut active_timers: HashMap<String, (AbortHandle, u64)> = HashMap::new();
        let mut pending_moves = HashMap::new();

        let mut join_set = JoinSet::new();

        loop {
            tokio::select! {
                Some(cmd) = cmd_rx.recv() => {
                    match cmd {
                        DebouncerCommand::Event { event } => {
                            let filter = watch_map.get_filter(&event.path);

                            if filter.contains(WatchFilter::IGNORE_DIRECTORIES)
                                && event.flag.contains(FsEventFlag::ITEM_IS_DIR)
                            {
                                continue;
                            }

                            if filter.contains(WatchFilter::IGNORE_FILES)
                                && event.flag.contains(FsEventFlag::ITEM_IS_FILE)
                            {
                                continue;
                            }

                            if event.flag.contains(FsEventFlag::ITEM_RENAMED) {
                                let is_destination = Path::new(&event.path).exists();
                                if is_destination {
                                    let matched_source = pending_moves.remove(&event.id).or_else(|| {
                                        if event.id == 0 {
                                            return None;
                                        }

                                        pending_moves.remove(&(event.id - 1))
                                    });

                                    if let Some(old_path) = matched_source {
                                        let mut old_matches = watch_map.get_matches(&old_path);
                                        let mut new_matches = watch_map.get_matches(&event.path);

                                        old_matches.sort_unstable();
                                        new_matches.sort_unstable();

                                        if old_matches == new_matches
                                            && let Some((abort_handle, _)) = active_timers.remove(&old_path)
                                        {
                                            abort_handle.abort();
                                        }
                                    }
                                } else {
                                    pending_moves.insert(event.id, event.path.clone());
                                }
                            }

                            if let Some((abort_handle, old_id)) = active_timers.remove(&event.path) {
                                abort_handle.abort();
                                pending_moves.remove(&old_id);
                            }

                            let path_clone = event.path.clone();
                            let abort_handle = join_set.spawn(async move {
                                sleep(DEBOUNCE_DURATION).await;
                                (path_clone, event.id)
                            });

                            active_timers.insert(event.path, (abort_handle, event.id));
                        }
                        DebouncerCommand::Watch { id, path, filter } => {
                            watch_map.insert(path, id, filter);
                        }
                        DebouncerCommand::Unwatch { id } => {
                            watch_map.remove_by_id(id);
                        }
                    }
                }

                Some(result) = join_set.join_next() => {
                    let (ref path, event_id) = match result {
                        Ok(result) => result,
                        _ => continue,
                    };

                    active_timers.remove(path);

                    pending_moves.remove(&event_id);

                    let matched_ids = watch_map.get_matches(path);

                    for watch_id in matched_ids {
                        let event = WatchEvent {
                            path: path.clone(),
                            watch_id,
                        };
                        let _ = tx.send(Ok(event));
                    }
                }
            }
        }
    })
}
