use color_eyre::eyre::Result;
use tokio::sync::{mpsc, oneshot};

use crate::{
    actor::{DatabaseActor, cmd::DatabaseCmd, run_database_actor},
    song::Song,
};

const DATABASE_ACTOR_KILLED: &str = "DatabaseActor task has been killed";

#[derive(Clone)]
pub struct DatabaseActorHandle {
    sender: mpsc::Sender<DatabaseCmd>,
}

impl Default for DatabaseActorHandle {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = DatabaseActor::new(receiver);
        tokio::spawn(run_database_actor(actor));

        Self { sender }
    }
}

impl DatabaseActorHandle {
    pub async fn get_songs(&self) -> Result<Vec<Song>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetSongs { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn load_database(&self) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::LoadDatabase { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }
}
