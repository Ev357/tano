use color_eyre::eyre::Result;
use tano_providers::local::parse_song::ParsedSong;
use tokio::sync::oneshot;

use crate::{
    album::Album,
    artist::Artist,
    local_song::{LocalSong, SyncLocalSong},
    song::Song,
};

pub enum DatabaseCmd {
    LoadDatabase {
        respond_to: oneshot::Sender<Result<()>>,
    },
    GetSongs {
        respond_to: oneshot::Sender<Result<Vec<Song>>>,
    },
    GetAlbums {
        respond_to: oneshot::Sender<Result<Vec<Album>>>,
    },
    GetArtists {
        respond_to: oneshot::Sender<Result<Vec<Artist>>>,
    },
    GetSongIds {
        respond_to: oneshot::Sender<Result<Vec<i64>>>,
    },
    GetSyncLocalSong {
        provider_id: u64,
        respond_to: oneshot::Sender<Result<Vec<SyncLocalSong>>>,
    },
    SyncLocalSongs {
        provider_id: u64,
        new_songs: Vec<ParsedSong>,
        updated_songs: Vec<(i64, ParsedSong)>,
        to_update_path: Vec<(i64, String)>,
        to_delete_ids: Vec<i64>,
        respond_to: oneshot::Sender<Result<()>>,
    },
    GetLocalSongByPath {
        provider_id: u64,
        path: String,
        respond_to: oneshot::Sender<Result<Option<LocalSong>>>,
    },
    GetLocalSongByInode {
        provider_id: u64,
        inode: i64,
        respond_to: oneshot::Sender<Result<Option<LocalSong>>>,
    },
    UpdateLocalSongPath {
        id: i64,
        path: String,
        respond_to: oneshot::Sender<Result<()>>,
    },
    InsertLocalSong {
        provider_id: u64,
        parsed_song: ParsedSong,
        respond_to: oneshot::Sender<Result<i64>>,
    },
    UpdateLocalSong {
        provider_id: u64,
        id: i64,
        parsed_song: ParsedSong,
        respond_to: oneshot::Sender<Result<()>>,
    },
    DeleteLocalSong {
        id: i64,
        respond_to: oneshot::Sender<Result<()>>,
    },
}
