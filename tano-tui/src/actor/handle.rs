use color_eyre::eyre::Result;
use tokio::sync::{mpsc, oneshot, watch};

use crate::{
    actor::{TuiActor, cmd::TuiCmd, run_tui_actor},
    model::TuiModel,
};

const TUI_ACTOR_KILLED: &str = "TuiActor task has been killed";

#[derive(Clone)]
pub struct TuiActorHandle {
    sender: mpsc::Sender<TuiCmd>,
}

impl TuiActorHandle {
    pub fn new<T: TuiModel>(model_rx: watch::Receiver<T>) -> Result<Self> {
        let (sender, receiver) = mpsc::channel(8);
        let actor = TuiActor::new(receiver, model_rx)?;
        tokio::spawn(run_tui_actor(actor));

        Ok(Self { sender })
    }

    pub async fn render(&self) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = TuiCmd::Render { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(TUI_ACTOR_KILLED)
    }

    pub async fn restore(&self) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = TuiCmd::Restore { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(TUI_ACTOR_KILLED)
    }
}
