use color_eyre::eyre::Result;
use tokio::sync::{mpsc, oneshot, watch};

use crate::{
    actor::{BackendActor, cmd::BackendCmd, msg::BackendMsg, run_backend_actor},
    model::BackendModel,
};

const BACKEND_ACTOR_KILLED: &str = "BackendActor task has been killed";

#[derive(Clone)]
pub struct BackendActorHandle {
    sender: mpsc::Sender<BackendCmd>,
}

impl BackendActorHandle {
    pub fn new<T: BackendModel>(
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<BackendMsg>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = BackendActor::new(receiver, model_rx, msg_tx);
        tokio::spawn(run_backend_actor(actor));

        Self { sender }
    }

    pub async fn restore(&self) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = BackendCmd::Restore { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(BACKEND_ACTOR_KILLED)
    }
}
