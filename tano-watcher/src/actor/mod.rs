use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

use color_eyre::eyre::{Result, eyre};
use notify::{
    Config, Event, EventKind, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher,
    event::{ModifyKind, RenameMode},
};
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
    watch_type::WatchType,
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
    watched_paths: HashMap<PathBuf, WatchType>,
    latest_value: Option<Event>,
}

impl<T: WatcherModel> WatcherActor<T> {
    pub fn new(
        receiver: mpsc::Receiver<WatcherCmd>,
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<WatcherMsg>,
    ) -> Result<Self> {
        let (notify_tx, notify_rx) = mpsc::channel(100);
        let watcher = RecommendedWatcher::new(
            move |event| {
                let _ = notify_tx.blocking_send(event);
            },
            Config::default(),
        )?;

        Ok(Self {
            receiver,
            model_rx,
            msg_tx,
            watcher,
            notify_rx,
            watched_paths: HashMap::new(),
            latest_value: None,
        })
    }

    async fn handle_command(&mut self, cmd: WatcherCmd) {
        match cmd {
            WatcherCmd::Watch {
                path,
                watch_type,
                respond_to,
            } => {
                let _ = respond_to.send(self.watch(path, watch_type));
            }
            WatcherCmd::Unwatch { path, respond_to } => {
                let _ = respond_to.send(self.unwatch(&path));
            }
        }
    }

    fn handle_update(&self) {
        // TODO: handle update
    }

    fn handle_debounce_event(&mut self, event: notify::Result<Event>) -> Result<()> {
        let event = event?;
        if self.is_event_relevant(&event) {
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
                let watch_type = self.watched_paths.get(&path).cloned();

                if let Some(watch_type) = watch_type {
                    let _ = self
                        .msg_tx
                        .send(WatcherMsg::FileChange { path, watch_type })
                        .await;
                }
            }
        }

        Ok(())
    }

    fn watch(&mut self, path: PathBuf, watch_type: WatchType) -> Result<()> {
        let parent = self.get_parent_dir(&path)?;

        self.watcher.watch(parent, RecursiveMode::NonRecursive)?;
        self.watched_paths.insert(path, watch_type);

        Ok(())
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        let parent = self.get_parent_dir(path)?;

        self.watcher.unwatch(parent)?;
        self.watched_paths.remove(path);

        Ok(())
    }

    fn get_parent_dir<'a>(&self, path: &'a Path) -> Result<&'a Path> {
        let parent = path
            .parent()
            .ok_or(eyre!("Unable to get parent directory of: {:?}", path))?;

        Ok(parent)
    }

    fn is_event_relevant(&self, event: &Event) -> bool {
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
            return false;
        }

        match event.paths.as_slice() {
            [path] => self.watched_paths.contains_key(path),
            [_, target] => self.watched_paths.contains_key(target),
            _ => false,
        }
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
                actor.handle_update();
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
