use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
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
    path_type::PathType,
    watch_entry::WatchEntry,
    watch_mode::WatchMode,
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
    latest_value: Option<(Event, WatchEntry)>,
    config_entry: WatchEntry,
}

impl<T: WatcherModel> WatcherActor<T> {
    pub fn new(
        receiver: mpsc::Receiver<WatcherCmd>,
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<WatcherMsg>,
        config_entry: WatchEntry,
    ) -> Result<Self> {
        let (notify_tx, notify_rx) = mpsc::channel(100);
        let mut watcher = RecommendedWatcher::new(
            move |event| {
                let _ = notify_tx.blocking_send(event);
            },
            Config::default(),
        )?;

        let parent = get_parent_dir(&config_entry.path)?;

        let watched_paths = if parent.try_exists()? {
            watcher.watch(parent, RecursiveMode::NonRecursive)?;

            HashMap::from([(parent.to_path_buf(), config_entry.clone())])
        } else {
            HashMap::new()
        };

        Ok(Self {
            receiver,
            model_rx,
            msg_tx,
            watcher,
            notify_rx,
            config_entry,
            watched_paths,
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

            let config_parent = get_parent_dir(&self.config_entry.path)?;
            let mut entries = if config_parent.try_exists()? {
                HashSet::from([self.config_entry.clone()])
            } else {
                HashSet::new()
            };

            for provider in providers {
                match provider {
                    ProviderType::Local(local_provider) => {
                        entries.insert(WatchEntry {
                            path: local_provider.config.path.clone(),
                            path_type: PathType::Directory,
                            watch_type: WatchType::Provider(WatchProvider::Local),
                            watch_mode: WatchMode::all(),
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
            self.unwatch(watch_entry)?;
        }

        for watch_entry in added {
            self.watch(watch_entry.clone())?;
        }

        Ok(())
    }

    fn handle_debounce_event(&mut self, event: notify::Result<Event>) -> Result<()> {
        let event = event?;
        if let Some(entry) = self.is_event_relevant(&event)? {
            self.latest_value = Some((event, entry));
        }

        Ok(())
    }

    async fn handle_event(&mut self) -> Result<()> {
        let (mut event, entry) = match self.latest_value.take() {
            Some(value) => value,
            None => return Ok(()),
        };

        let path = match event.paths.len() {
            1 | 2 => match event.paths.pop() {
                Some(path) => path,
                None => return Ok(()),
            },
            _ => return Ok(()),
        };

        let _ = self
            .msg_tx
            .send(WatcherMsg::FileChange {
                path,
                watch_type: entry.watch_type,
            })
            .await;

        Ok(())
    }

    fn watch(&mut self, watch_entry: WatchEntry) -> Result<()> {
        let watch_path = if matches!(watch_entry.path_type, PathType::File) {
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

    fn unwatch(&mut self, watch_entry: &WatchEntry) -> Result<()> {
        let watch_path = if matches!(watch_entry.path_type, PathType::File) {
            get_parent_dir(&watch_entry.path)?
        } else {
            &watch_entry.path
        };

        self.watcher.unwatch(watch_path)?;
        self.watched_paths.remove(watch_path);

        Ok(())
    }

    fn is_event_relevant(&self, event: &Event) -> Result<Option<WatchEntry>> {
        let path = match event.paths.as_slice() {
            [path] | [_, path] => path,
            _ => return Ok(None),
        };

        let entry = match self.watched_paths.get(path) {
            Some(entry) => entry,
            None => {
                let parent = get_parent_dir(path)?;

                match self.watched_paths.get(parent) {
                    Some(entry) => entry,
                    None => return Ok(None),
                }
            }
        };

        let matches_create = entry.watch_mode.create
            && matches!(
                event.kind,
                EventKind::Create(_)
                    | EventKind::Modify(ModifyKind::Name(RenameMode::To | RenameMode::Both))
            );

        let matches_modify = entry.watch_mode.modify
            && matches!(
                event.kind,
                EventKind::Modify(ModifyKind::Data(_) | ModifyKind::Any | ModifyKind::Other)
            );

        let matches_remove = entry.watch_mode.remove && matches!(event.kind, EventKind::Remove(_));

        if !matches_create && !matches_modify && !matches_remove {
            return Ok(None);
        }

        Ok(Some(entry.clone()))
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
