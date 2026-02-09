use color_eyre::eyre::Result;
use tokio::sync::oneshot;

use crate::song::{CreateSong, Song};

pub enum DatabaseCmd {
    LoadDatabase {
        respond_to: oneshot::Sender<Result<()>>,
    },
    GetSongs {
        respond_to: oneshot::Sender<Result<Vec<Song>>>,
    },
    SyncSongs {
        songs: Vec<CreateSong>,
        respond_to: oneshot::Sender<Result<()>>,
    },
}
