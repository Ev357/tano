use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    time::Duration,
};

use color_eyre::eyre::Result;
use notify::{
    Config, Event, EventKind, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher,
    event::{ModifyKind, RenameMode},
};
use tano_providers::ProviderType;
use tano_shared::get_parent_dir::get_parent_dir;
use tokio::{
    sync::{
        mpsc,
        watch::{self},
    },
    time::sleep,
};

use crate::{
    actor::{cmd::WatcherCmd, msg::WatcherMsg},
    model::WatcherModel,
    watch_entry::WatchEntry,
    watch_type::{WatchProvider, WatchType},
};

pub mod cmd;
pub mod handle;
pub mod msg;

pub struct WatcherActor<T: WatcherModel> {
    receiver: mpsc::Receiver<WatcherCmd>,
    model_rx: watch::Receiver<T>,
    msg_tx: mpsc::Sender<WatcherMsg>,
    watcher: INotifyWatcher,
    notify_rx: mpsc::Receiver<notify::Result<Event>>,
    watched_paths: HashMap<PathBuf, WatchEntry>,
    latest_value: Option<Event>,
    config_path: PathBuf,
}

impl<T: WatcherModel> WatcherActor<T> {
    pub fn new(
        receiver: mpsc::Receiver<WatcherCmd>,
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<WatcherMsg>,
        config_path: PathBuf,
    ) -> Result<Self> {
        let (notify_tx, notify_rx) = mpsc::channel(100);
        let mut watcher = RecommendedWatcher::new(
            move |event| {
                let _ = notify_tx.blocking_send(event);
            },
            Config::default(),
        )?;

        watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

        Ok(Self {
            receiver,
            model_rx,
            msg_tx,
            watcher,
            notify_rx,
            config_path,
            watched_paths: HashMap::new(),
            latest_value: None,
        })
    }

    async fn handle_command(&mut self, _cmd: WatcherCmd) {
        // TODO: handle update
    }

    fn handle_update(&mut self) -> Result<()> {
        let new_watch_entries = {
            let model = self.model_rx.borrow();
            let providers = model.providers();

            let mut entries = HashSet::from([WatchEntry {
                path: self.config_path.clone(),
                watch_type: WatchType::Config,
            }]);

            for provider in providers {
                match provider {
                    ProviderType::Local(local_provider) => {
                        let path_str = local_provider.config.path.to_string_lossy();
                        let expanded_cow = shellexpand::full(&path_str)?;

                        let path = PathBuf::from(expanded_cow.as_ref());

                        entries.insert(WatchEntry {
                            path,
                            watch_type: WatchType::Provider(WatchProvider::Local),
                        });
                    }
                }
            }
            entries
        };

        let watch_entries: HashSet<WatchEntry> = self.watched_paths.values().cloned().collect();

        let added: Vec<&WatchEntry> = new_watch_entries.difference(&watch_entries).collect();
        let removed: Vec<&WatchEntry> = watch_entries.difference(&new_watch_entries).collect();

        for watch_entry in removed {
            self.unwatch(&watch_entry.path)?;
        }

        for watch_entry in added {
            self.watch(watch_entry.clone())?;
        }

        Ok(())
    }

    fn handle_debounce_event(&mut self, event: notify::Result<Event>) -> Result<()> {
        let event = event?;
        if self.is_event_relevant(&event)? {
            self.latest_value = Some(event);
        }

        Ok(())
    }

    async fn handle_event(&mut self) -> Result<()> {
        if let Some(mut event) = self.latest_value.take() {
            let path = match event.paths.len() {
                1 => event.paths.pop(),
                2 => event.paths.pop(),
                _ => None,
            };

            if let Some(path) = path {
                let watch_path = if path.is_file() {
                    get_parent_dir(&path)?
                } else {
                    &path
                };

                let watch_type = self.watched_paths.get(watch_path).cloned();

                if let Some(watch_entry) = watch_type {
                    let _ = self
                        .msg_tx
                        .send(WatcherMsg::FileChange {
                            path,
                            watch_type: watch_entry.watch_type,
                        })
                        .await;
                }
            }
        }

        Ok(())
    }

    fn watch(&mut self, watch_entry: WatchEntry) -> Result<()> {
        let watch_path = if watch_entry.path.is_file() {
            get_parent_dir(&watch_entry.path)?
        } else {
            &watch_entry.path
        };

        self.watcher
            .watch(watch_path, RecursiveMode::NonRecursive)?;
        self.watched_paths
            .insert(watch_path.to_path_buf(), watch_entry);

        Ok(())
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        let watch_path = if path.is_file() {
            get_parent_dir(path)?
        } else {
            path
        };

        self.watcher.unwatch(watch_path)?;
        self.watched_paths.remove(watch_path);

        Ok(())
    }

    fn is_event_relevant(&self, event: &Event) -> Result<bool> {
        let is_kind_relevant = matches!(
            event.kind,
            EventKind::Create(_)
                | EventKind::Modify(
                    ModifyKind::Data(_)
                        | ModifyKind::Name(RenameMode::To | RenameMode::Both)
                        | ModifyKind::Any
                        | ModifyKind::Other
                )
        );

        if !is_kind_relevant {
            return Ok(false);
        }

        let result = match event.paths.as_slice() {
            [path] => {
                let parent = get_parent_dir(path)?;
                self.watched_paths.contains_key(path) || self.watched_paths.contains_key(parent)
            }
            [_, target] => {
                let parent = get_parent_dir(target)?;
                self.watched_paths.contains_key(target) || self.watched_paths.contains_key(parent)
            }
            _ => false,
        };

        Ok(result)
    }
}

pub async fn run_watcher_actor<T: WatcherModel>(mut actor: WatcherActor<T>) -> Result<()> {
    let debounce_duration = Duration::from_millis(50);

    loop {
        tokio::select! {
            Some(cmd) = actor.receiver.recv() => {
                actor.handle_command(cmd).await;
            }
            Ok(_) = actor.model_rx.changed() => {
                actor.handle_update()?;
            }
            Some(event) = actor.notify_rx.recv() => {
                actor.handle_debounce_event(event)?;
            },
            _ = sleep(debounce_duration), if actor.latest_value.is_some() => {
                actor.handle_event().await?;
            },
            else => break,
        }
    }

    Ok(())
}
