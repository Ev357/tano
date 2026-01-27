use std::str::FromStr;

use color_eyre::eyre::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tano_shared::get_data_dir::get_data_dir;
use tokio::sync::mpsc;

use crate::{actor::cmd::DatabaseCmd, song::Song};

pub mod cmd;
pub mod handle;
pub mod mgs;

pub struct DatabaseActor {
    receiver: mpsc::Receiver<DatabaseCmd>,
    pool: Option<SqlitePool>,
}

impl DatabaseActor {
    pub fn new(receiver: mpsc::Receiver<DatabaseCmd>) -> Self {
        Self {
            receiver,
            pool: None,
        }
    }

    async fn handle_command(&mut self, cmd: DatabaseCmd) {
        match cmd {
            DatabaseCmd::LoadDatabase { respond_to } => {
                let _ = respond_to.send(self.load_database().await);
            }
            DatabaseCmd::GetSongs { respond_to } => {
                let _ = respond_to.send(self.get_songs().await);
            }
        }
    }

    async fn get_songs(&self) -> Result<Vec<Song>> {
        let songs = sqlx::query_as!(Song, "SELECT id, title, provider_id, path FROM songs")
            .fetch_all(self.pool.as_ref().unwrap())
            .await?;

        Ok(songs)
    }

    async fn load_database(&mut self) -> Result<()> {
        let data_dir = get_data_dir()?;

        let database_path = &data_dir.join("database.db").to_string_lossy().to_string();

        let connection_options =
            SqliteConnectOptions::from_str(database_path)?.create_if_missing(true);

        let pool = SqlitePool::connect_with(connection_options).await?;

        sqlx::migrate!().run(&pool).await?;

        self.pool = Some(pool);

        Ok(())
    }
}

pub async fn run_database_actor(mut actor: DatabaseActor) {
    while let Some(cmd) = actor.receiver.recv().await {
        actor.handle_command(cmd).await;
    }
}
