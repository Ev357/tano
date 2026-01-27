use color_eyre::eyre::Result;
use tokio::sync::{mpsc, oneshot};

use crate::{
    actor::{ConfigActor, cmd::ConfigCmd, run_config_actor},
    config::Config,
};

const CONFIG_ACTOR_KILLED: &str = "ConfigActor task has been killed";

#[derive(Clone)]
pub struct ConfigActorHandle {
    sender: mpsc::Sender<ConfigCmd>,
}

impl Default for ConfigActorHandle {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = ConfigActor::new(receiver);
        tokio::spawn(run_config_actor(actor));

        Self { sender }
    }
}

impl ConfigActorHandle {
    pub async fn load_config(&self) -> Result<Config> {
        let (send, recv) = oneshot::channel();
        let cmd = ConfigCmd::LoadConfig { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(CONFIG_ACTOR_KILLED)
    }
}
