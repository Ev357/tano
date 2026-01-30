use std::path::PathBuf;

use color_eyre::eyre::Result;
use tokio::sync::{mpsc, oneshot, watch};

use crate::{
    actor::{WatcherActor, cmd::WatcherCmd, msg::WatcherMsg, run_watcher_actor},
    model::WatcherModel,
    watch_type::WatchType,
};

const BACKEND_ACTOR_KILLED: &str = "WatcherActor task has been killed";

#[derive(Clone)]
pub struct WatcherActorHandle {
    sender: mpsc::Sender<WatcherCmd>,
}

impl WatcherActorHandle {
    pub fn new<T: WatcherModel>(
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<WatcherMsg>,
    ) -> Result<Self> {
        let (sender, receiver) = mpsc::channel(8);
        let actor = WatcherActor::new(receiver, model_rx, msg_tx)?;
        tokio::spawn(run_watcher_actor(actor));

        Ok(Self { sender })
    }

    pub async fn watch(&self, path: PathBuf, watch_type: WatchType) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = WatcherCmd::Watch {
            path,
            watch_type,
            respond_to: send,
        };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(BACKEND_ACTOR_KILLED)
    }
}
