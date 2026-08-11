use std::{
    collections::{HashMap, HashSet},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

use color_eyre::eyre::Result;
use tano_providers::local::parse_song::parse_song;
use tokio::{fs, task::JoinSet};

use crate::update::handles::Handles;

pub async fn full_sync(handles: Handles, provider_id: u64, provider_path: PathBuf) -> Result<()> {
    let db_songs = handles.database.get_sync_local_songs(provider_id).await?;

    let mut path_map = HashMap::with_capacity(db_songs.len());
    let mut inode_map = HashMap::with_capacity(db_songs.len());
    let mut unverified_ids = HashSet::with_capacity(db_songs.len());

    for song in &db_songs {
        path_map.insert(song.path.as_str(), song);
        inode_map.insert(song.inode, song);
        unverified_ids.insert(song.song_id);
    }

    let mut to_create = Vec::new();
    let mut to_update = Vec::new();
    let mut to_update_path = Vec::new();

    let mut entries = fs::read_dir(&provider_path).await?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        if entry.path().is_dir() {
            continue;
        }

        let path_str = fs::canonicalize(entry.path()).await?;
        let path_str = path_str.to_string_lossy().to_string();
        let meta = entry.metadata().await?;
        let inode = meta.ino() as i64;
        let mtime = meta.mtime();
        let size = meta.size() as i64;

        if let Some(song) = path_map.get(path_str.as_str()) {
            unverified_ids.remove(&song.song_id);
            if song.inode != inode || song.mtime != mtime || song.size != size {
                to_update.push((song.song_id, path_str));
            }
        } else if let Some(song) = inode_map.get(&inode) {
            unverified_ids.remove(&song.song_id);
            if song.mtime != mtime || song.size != size {
                to_update.push((song.song_id, path_str));
            } else {
                to_update_path.push((song.song_id, path_str));
            }
        } else {
            to_create.push(path_str);
        }
    }

    let to_delete_ids: Vec<i64> = unverified_ids.into_iter().collect();

    let mut insert_tasks = JoinSet::new();
    for path in to_create {
        insert_tasks.spawn(async move { parse_song(&path).await });
    }

    let mut update_tasks = JoinSet::new();
    for (id, path) in to_update {
        update_tasks.spawn(async move {
            let parsed = parse_song(&path).await;
            (id, parsed)
        });
    }

    let mut new_songs = Vec::new();
    while let Some(res) = insert_tasks.join_next().await {
        if let Ok(Ok(parsed)) = res {
            new_songs.push(parsed);
        }
    }

    let mut updated_songs = Vec::new();
    while let Some(res) = update_tasks.join_next().await {
        if let Ok((id, Ok(parsed))) = res {
            updated_songs.push((id, parsed));
        }
    }

    if new_songs.is_empty()
        && updated_songs.is_empty()
        && to_update_path.is_empty()
        && to_delete_ids.is_empty()
    {
        return Ok(());
    }

    let _ = handles
        .database
        .sync_local_songs(
            provider_id,
            new_songs,
            updated_songs,
            to_update_path,
            to_delete_ids,
        )
        .await;

    Ok(())
}
