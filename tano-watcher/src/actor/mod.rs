use std::{collections::HashSet, path::PathBuf};

use color_eyre::eyre::Result;
use tokio::sync::{
    mpsc,
    watch::{self},
};

use crate::{
    RecommendedWatcher,
    actor::{cmd::WatcherCmd, msg::WatcherMsg},
    model::WatcherModel,
    recommended_watcher,
    watch_entry::WatchEntry,
    watch_event::WatchEvent,
    watcher::Watcher,
};

pub mod cmd;
pub mod handle;
pub mod msg;

pub struct WatcherActor<T: WatcherModel> {
    receiver: mpsc::Receiver<WatcherCmd>,
    model_rx: watch::Receiver<T>,
    msg_tx: mpsc::Sender<WatcherMsg>,
    watcher: RecommendedWatcher,
    notify_rx: mpsc::UnboundedReceiver<Result<WatchEvent>>,
    entries: HashSet<WatchEntry>,
}

impl<T: WatcherModel> WatcherActor<T> {
    pub fn new(
        receiver: mpsc::Receiver<WatcherCmd>,
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<WatcherMsg>,
    ) -> Result<Self> {
        let (notify_tx, notify_rx) = mpsc::unbounded_channel();
        let watcher = recommended_watcher(notify_tx)?;

        Ok(Self {
            receiver,
            model_rx,
            msg_tx,
            watcher,
            notify_rx,
            entries: HashSet::new(),
        })
    }

    async fn handle_command(&mut self, _cmd: WatcherCmd) {
        unimplemented!()
    }

    fn handle_update(&mut self) -> Result<()> {
        let new_watch_entries = self.model_rx.borrow().entries();

        let added: Vec<WatchEntry> = new_watch_entries
            .difference(&self.entries)
            .cloned()
            .collect();
        let removed: Vec<WatchEntry> = self
            .entries
            .difference(&new_watch_entries)
            .cloned()
            .collect();

        for watch_entry in removed {
            self.unwatch(&watch_entry)?;
        }

        for watch_entry in added {
            self.watch(watch_entry)?;
        }

        Ok(())
    }

    async fn handle_event(&mut self, event: Result<WatchEvent>) -> Result<()> {
        let event = event?;
        let path = PathBuf::from(event.path);

        let _ = self
            .msg_tx
            .send(WatcherMsg::WatchEvent {
                path,
                watch_id: event.watch_id,
            })
            .await;

        Ok(())
    }

    fn watch(&mut self, watch_entry: WatchEntry) -> Result<()> {
        self.watcher.watch(watch_entry.clone())?;
        self.entries.insert(watch_entry);

        Ok(())
    }

    fn unwatch(&mut self, watch_entry: &WatchEntry) -> Result<()> {
        self.watcher.unwatch(&watch_entry.id)?;
        self.entries.remove(watch_entry);

        Ok(())
    }
}

pub async fn run_watcher_actor<T: WatcherModel>(mut actor: WatcherActor<T>) {
    loop {
        let result = tokio::select! {
            Some(cmd) = actor.receiver.recv() => {
                actor.handle_command(cmd).await;

                Ok(())
            }
            Ok(_) = actor.model_rx.changed() => {
                actor.handle_update()
            }
            Some(event) = actor.notify_rx.recv() => {
                actor.handle_event(event).await
            },
            else => return,
        };

        if let Err(error) = result {
            let _ = actor.msg_tx.send(WatcherMsg::Error(error)).await;
            return;
        }
    }
}
