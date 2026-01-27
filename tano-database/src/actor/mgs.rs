use color_eyre::eyre::Result;

use crate::song::Song;

#[derive(Debug)]
pub enum DatabaseMsg {
    SongsLoaded { songs: Result<Vec<Song>> },
}
