use color_eyre::eyre::Result;
use tano_providers::local::parse_song::ParsedSong;
use tokio::sync::{mpsc, oneshot};

use crate::{
    actor::{DatabaseActor, cmd::DatabaseCmd, run_database_actor},
    album::Album,
    artist::Artist,
    local_song::{LocalSong, SyncLocalSong},
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

    pub async fn get_albums(&self) -> Result<Vec<Album>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetAlbums { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn get_album_artists(&self, album_id: i64) -> Result<Vec<Artist>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetAlbumArtists {
            album_id,
            respond_to: send,
        };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn get_album(&self, id: i64) -> Result<Option<Album>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetAlbum {
            id,
            respond_to: send,
        };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn get_album_songs(&self, album_id: i64) -> Result<Vec<Song>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetAlbumSongs {
            album_id,
            respond_to: send,
        };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn get_artists(&self) -> Result<Vec<Artist>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetArtists { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn get_sync_local_songs(&self, provider_id: u64) -> Result<Vec<SyncLocalSong>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetSyncLocalSong {
            respond_to: send,
            provider_id,
        };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn get_song_ids(&self) -> Result<Vec<i64>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetSongIds { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn sync_local_songs(
        &self,
        provider_id: u64,
        new_songs: Vec<ParsedSong>,
        updated_songs: Vec<(i64, ParsedSong)>,
        to_update_path: Vec<(i64, String)>,
        to_delete_ids: Vec<i64>,
    ) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::SyncLocalSongs {
            provider_id,
            new_songs,
            updated_songs,
            to_update_path,
            to_delete_ids,
            respond_to: send,
        };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn load_database(&self) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::LoadDatabase { respond_to: send };

        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn get_local_song_by_path(
        &self,
        provider_id: u64,
        path: String,
    ) -> Result<Option<LocalSong>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetLocalSongByPath {
            provider_id,
            path,
            respond_to: send,
        };
        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn get_local_song_by_inode(
        &self,
        provider_id: u64,
        inode: i64,
    ) -> Result<Option<LocalSong>> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::GetLocalSongByInode {
            provider_id,
            inode,
            respond_to: send,
        };
        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn update_local_song_path(&self, id: i64, path: String) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::UpdateLocalSongPath {
            id,
            path,
            respond_to: send,
        };
        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn insert_local_song(
        &self,
        provider_id: u64,
        parsed_song: ParsedSong,
    ) -> Result<i64> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::InsertLocalSong {
            provider_id,
            parsed_song,
            respond_to: send,
        };
        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn update_local_song(
        &self,
        provider_id: u64,
        id: i64,
        parsed_song: ParsedSong,
    ) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::UpdateLocalSong {
            provider_id,
            id,
            parsed_song,
            respond_to: send,
        };
        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }

    pub async fn delete_local_song(&self, id: i64) -> Result<()> {
        let (send, recv) = oneshot::channel();
        let cmd = DatabaseCmd::DeleteLocalSong {
            id,
            respond_to: send,
        };
        let _ = self.sender.send(cmd).await;
        recv.await.expect(DATABASE_ACTOR_KILLED)
    }
}
