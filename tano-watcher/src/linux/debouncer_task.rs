use std::collections::HashMap;

use color_eyre::eyre::Result;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::{AbortHandle, JoinHandle, JoinSet},
    time::sleep,
};

use crate::{
    constants::DEBOUNCE_DURATION,
    linux::inotify_event::{INotifyEvent, INotifyMask},
    watch_event::WatchEvent,
    watch_id::WatchId,
};

#[derive(Debug)]
pub enum DebouncerCommand {
    Event {
        event: INotifyEvent,
    },
    Watch {
        wd: i32,
        path: String,
        watch_id: WatchId,
    },
    Unwatch {
        wd: i32,
    },
}

pub fn debouncer_task(
    mut cmd_rx: UnboundedReceiver<DebouncerCommand>,
    tx: UnboundedSender<Result<WatchEvent>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut watch_map = HashMap::new();
        let mut active_timers: HashMap<(i32, String), AbortHandle> = HashMap::new();
        let mut pending_moves = HashMap::new();

        let mut join_set = JoinSet::new();

        loop {
            tokio::select! {
                Some(cmd) = cmd_rx.recv() => {
                    match cmd {
                        DebouncerCommand::Event { event } => {
                            let file_name = match &event.name {
                                Some(name) => name.clone(),
                                None => continue,
                            };

                            if event.mask.contains(INotifyMask::ISDIR) {
                                continue;
                            }

                            let key = (event.wd, file_name.clone());

                            if event.mask.contains(INotifyMask::MOVED_FROM) && event.cookie != 0 {
                                pending_moves.insert(event.cookie, key.clone());
                            }

                            if event.mask.contains(INotifyMask::MOVED_TO)
                                && event.cookie != 0
                                && let Some(old_key @ (wd, _)) = pending_moves.remove(&event.cookie)
                                && wd == event.wd
                                && let Some(abort_handle) = active_timers.remove(&old_key)
                            {
                                abort_handle.abort();
                            }

                            if let Some(abort_handle) = active_timers.remove(&key) {
                                abort_handle.abort();
                            }

                            let abort_handle = join_set.spawn(async move {
                                sleep(DEBOUNCE_DURATION).await;
                                (key, event.cookie)
                            });

                            active_timers.insert((event.wd, file_name), abort_handle);
                        }
                        DebouncerCommand::Watch { wd, path, watch_id } => {
                            watch_map.insert(wd, (path, watch_id));
                        }
                        DebouncerCommand::Unwatch { wd } => {
                            watch_map.remove(&wd);

                            active_timers.retain(|(timer_wd, _), handle| {
                                if *timer_wd == wd {
                                    handle.abort();
                                    return false;
                                }
                                true
                            });

                            pending_moves.retain(|_, (move_wd, _)| *move_wd != wd);
                        }
                    }
                }

                Some(result) = join_set.join_next() => {
                    let (ref key, cookie) = match result {
                        Ok(result) => result,
                        _ => continue,
                    };

                    active_timers.remove(key);

                    if cookie != 0 {
                        pending_moves.remove(&cookie);
                    }

                    let (wd, file_name) = key;
                    if let Some((base_path, watch_id)) = watch_map.get(wd) {
                        let full_path = format!("{}/{}", base_path.trim_end_matches('/'), file_name);
                        let event = WatchEvent {
                            path: full_path,
                            watch_id: *watch_id,
                        };
                        let _ = tx.send(Ok(event));
                    }
                }
            }
        }
    })
}
