use color_eyre::eyre::Result;
use tokio::sync::{mpsc, oneshot, watch};

use crate::{
    actor::{BackendActor, cmd::BackendCmd, msg::BackendMsg, run_backend_actor},
    model::BackendModel,
};

#[allow(unused)]
const BACKEND_ACTOR_KILLED: &str = "BackendActor task has been killed";

#[derive(Clone)]
pub struct BackendActorHandle {
    sender: mpsc::Sender<BackendCmd>,
}

impl BackendActorHandle {
    pub fn new<T: BackendModel>(
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<BackendMsg>,
    ) -> Result<Self> {
        let (sender, receiver) = mpsc::channel(8);
        let actor = BackendActor::new(receiver, model_rx, msg_tx)?;
        tokio::spawn(run_backend_actor(actor));

        Ok(Self { sender })
    }

    pub async fn suspend(&self) {
        let (send, recv) = oneshot::channel();
        let _ = self
            .sender
            .send(BackendCmd::Suspend { respond_to: send })
            .await;
        let _ = recv.await;
    }
}
