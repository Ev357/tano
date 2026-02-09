use std::str::FromStr;

use color_eyre::eyre::Result;
use sqlx::{QueryBuilder, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};
use tano_shared::get_data_dir::get_data_dir;
use tokio::sync::mpsc;

use crate::{
    actor::cmd::DatabaseCmd,
    song::{CreateSong, Song},
};

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
            DatabaseCmd::SyncSongs { songs, respond_to } => {
                let _ = respond_to.send(self.sync_songs(&songs).await);
            }
        }
    }

    async fn get_songs(&self) -> Result<Vec<Song>> {
        let songs = sqlx::query_as!(
            Song,
            "SELECT id, title, provider_id, path FROM songs ORDER BY title"
        )
        .fetch_all(self.pool.as_ref().unwrap())
        .await?;

        Ok(songs)
    }

    async fn sync_songs(&self, create_songs: &[CreateSong]) -> Result<()> {
        let mut tx = self.pool.as_ref().unwrap().begin().await?;

        sqlx::query("CREATE TEMPORARY TABLE batch_sync (title TEXT, provider_id TEXT, path TEXT)")
            .execute(&mut *tx)
            .await?;

        if !create_songs.is_empty() {
            let mut query_builder: QueryBuilder<Sqlite> =
                QueryBuilder::new("INSERT INTO batch_sync (title, provider_id, path) ");

            query_builder.push_values(create_songs, |mut batch, song| {
                batch
                    .push_bind(&song.title)
                    .push_bind(&song.provider_id)
                    .push_bind(&song.path);
            });

            query_builder.build().execute(&mut *tx).await?;
        }

        sqlx::query(
            "
            DELETE FROM songs
            WHERE path NOT IN (SELECT path FROM batch_sync)
            ",
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "
            INSERT OR IGNORE INTO songs (title, provider_id, path)
            SELECT title, provider_id, path FROM batch_sync
            ",
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query("DROP TABLE batch_sync")
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
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
