use color_eyre::eyre::Result;
use tokio::sync::{mpsc, watch};

use crate::{
    actor::{WatcherActor, cmd::WatcherCmd, msg::WatcherMsg, run_watcher_actor},
    model::WatcherModel,
};

#[allow(unused)]
const BACKEND_ACTOR_KILLED: &str = "WatcherActor task has been killed";

#[derive(Debug, Clone)]
pub struct WatcherActorHandle {
    #[allow(unused)]
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
}
