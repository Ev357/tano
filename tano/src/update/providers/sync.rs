use std::{io::ErrorKind, os::unix::fs::MetadataExt, path::PathBuf};

use color_eyre::eyre::Result;
use tano_providers::local::parse_song::parse_song;
use tokio::fs;

use crate::Handles;

pub async fn sync(handles: Handles, provider_id: u64, path: PathBuf) -> Result<()> {
    let song_future = handles
        .database
        .get_local_song_by_path(provider_id, path.to_string_lossy().to_string());
    let metadata_future = fs::metadata(&path);

    let (song_result, metadata_result) = tokio::join!(song_future, metadata_future);

    let local_song = song_result?;

    let metadata = match metadata_result {
        Ok(meta) => Some(meta),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => None,
            _ => return Err(error.into()),
        },
    };

    match (metadata, local_song) {
        (Some(metadata), Some(local_song)) => {
            if metadata.mtime() == local_song.mtime
                && metadata.size() == local_song.size as u64
                && metadata.ino() == local_song.inode as u64
            {
                return Ok(());
            }

            let parsed_song = match parse_song(&path).await {
                Ok(parsed_song) => parsed_song,
                _ => return Ok(()),
            };

            let _ = handles
                .database
                .update_local_song(provider_id, local_song.song_id, parsed_song)
                .await;
        }
        (Some(metadata), None) => {
            let local_song = handles
                .database
                .get_local_song_by_inode(provider_id, metadata.ino() as i64)
                .await
                .unwrap_or(None);

            if let Some(local_song) = local_song {
                if metadata.mtime() != local_song.mtime || metadata.size() != local_song.size as u64
                {
                    let parsed_song = match parse_song(&path).await {
                        Ok(parsed_song) => parsed_song,
                        _ => return Ok(()),
                    };

                    let _ = handles
                        .database
                        .update_local_song(provider_id, local_song.song_id, parsed_song)
                        .await;
                } else {
                    let _ = handles
                        .database
                        .update_local_song_path(
                            local_song.song_id,
                            path.to_string_lossy().to_string(),
                        )
                        .await;
                }
            } else {
                let parsed_song = match parse_song(&path).await {
                    Ok(parsed_song) => parsed_song,
                    _ => return Ok(()),
                };

                let _ = handles
                    .database
                    .insert_local_song(provider_id, parsed_song)
                    .await;
            }
        }
        (None, Some(local_song)) => {
            let _ = handles.database.delete_local_song(local_song.song_id).await;
        }
        _ => {}
    }

    Ok(())
}
