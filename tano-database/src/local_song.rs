use sqlx::prelude::FromRow;

define_entity!(
    CreateLocalSong,
    LocalSong {
        song_id: i64,
        path: String,
        inode: i64,
        mtime: i64,
        size: i64,
        format: String,
    }
);

#[derive(Debug, FromRow)]
pub struct SyncLocalSong {
    pub song_id: i64,
    pub path: String,
    pub inode: i64,
    pub mtime: i64,
    pub size: i64,
}
