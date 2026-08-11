use std::str::FromStr;

use color_eyre::eyre::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tano_providers::local::parse_song::ParsedSong;
use tano_shared::get_data_dir::get_data_dir;
use tokio::{fs, sync::mpsc};

use crate::{actor::cmd::DatabaseCmd, db};

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
            DatabaseCmd::GetSongIds { respond_to } => {
                let _ = respond_to.send(self.get_song_ids().await);
            }
            DatabaseCmd::GetSyncLocalSong {
                respond_to,
                provider_id,
            } => {
                let _ = respond_to.send(self.get_sync_local_songs(provider_id).await);
            }
            DatabaseCmd::SyncLocalSongs {
                provider_id,
                new_songs,
                updated_songs,
                to_update_path,
                to_delete_ids,
                respond_to,
            } => {
                let _ = respond_to.send(
                    self.sync_local_songs(
                        provider_id,
                        new_songs,
                        updated_songs,
                        to_update_path,
                        to_delete_ids,
                    )
                    .await,
                );
            }
            DatabaseCmd::GetLocalSongByPath {
                provider_id,
                path,
                respond_to,
            } => {
                let _ = respond_to.send(
                    db::get_local_song_by_path(self.pool.as_ref().unwrap(), provider_id, &path)
                        .await,
                );
            }
            DatabaseCmd::GetLocalSongByInode {
                provider_id,
                inode,
                respond_to,
            } => {
                let _ = respond_to.send(
                    db::get_local_song_by_inode(self.pool.as_ref().unwrap(), provider_id, inode)
                        .await,
                );
            }
            DatabaseCmd::UpdateLocalSongPath {
                id,
                path,
                respond_to,
            } => {
                let _ = respond_to
                    .send(db::update_local_song_path(self.pool.as_ref().unwrap(), id, &path).await);
            }
            DatabaseCmd::InsertLocalSong {
                provider_id,
                parsed_song,
                respond_to,
            } => {
                let _ = respond_to.send(
                    db::insert_parsed_song(self.pool.as_ref().unwrap(), provider_id, &parsed_song)
                        .await,
                );
            }
            DatabaseCmd::UpdateLocalSong {
                provider_id,
                id,
                parsed_song,
                respond_to,
            } => {
                let _ = respond_to.send(
                    db::update_parsed_song(
                        self.pool.as_ref().unwrap(),
                        provider_id,
                        id,
                        &parsed_song,
                    )
                    .await,
                );
            }
            DatabaseCmd::DeleteLocalSong { id, respond_to } => {
                let _ =
                    respond_to.send(db::delete_parsed_song(self.pool.as_ref().unwrap(), id).await);
            }
        }
    }

    async fn get_songs(&self) -> Result<Vec<crate::song::Song>> {
        db::get_songs(self.pool.as_ref().unwrap()).await
    }

    async fn get_song_ids(&self) -> Result<Vec<i64>> {
        db::get_song_ids(self.pool.as_ref().unwrap()).await
    }

    async fn get_sync_local_songs(
        &self,
        provider_id: u64,
    ) -> Result<Vec<crate::local_song::SyncLocalSong>> {
        db::get_sync_local_songs(self.pool.as_ref().unwrap(), provider_id).await
    }

    async fn sync_local_songs(
        &self,
        provider_id: u64,
        new_songs: Vec<ParsedSong>,
        updated_songs: Vec<(i64, ParsedSong)>,
        to_update_path: Vec<(i64, String)>,
        to_delete_ids: Vec<i64>,
    ) -> Result<()> {
        db::sync_local_songs(
            self.pool.as_ref().unwrap(),
            provider_id,
            new_songs,
            updated_songs,
            to_update_path,
            to_delete_ids,
        )
        .await
    }

    async fn load_database(&mut self) -> Result<()> {
        let data_dir = get_data_dir()?;
        fs::create_dir_all(&data_dir).await?;

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
