use std::{fs::File, os::unix::fs::MetadataExt, path::Path};

use color_eyre::eyre::{Result, eyre};
use lofty::{
    file::{AudioFile, TaggedFileExt},
    tag::{Accessor, ItemKey},
};
use tokio::{fs, task};

use crate::local::get_format::get_format;

#[derive(Debug, Clone)]
pub struct ParsedSong {
    pub title: String,
    pub path: String,
    pub inode: i64,
    pub mtime: i64,
    pub size: i64,
    pub format: String,
    pub album_title: String,
    pub year: Option<i64>,
    pub artist_name: String,
    pub album_artist_name: String,
    pub track_number: Option<i64>,
    pub duration: i64,
}

pub async fn parse_song<T: AsRef<Path>>(path: T) -> Result<ParsedSong> {
    let path_ref = path.as_ref();
    let path_owned = path_ref.to_path_buf();

    let (metadata, tagged_file) = task::spawn_blocking(move || -> Result<_> {
        let mut file = File::open(&path_owned)?;
        let metadata = file.metadata()?;
        let tagged_file = lofty::read_from(&mut file)?;

        Ok((metadata, tagged_file))
    })
    .await??;

    let format = get_format(&tagged_file.file_type())?.to_string();

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())
        .ok_or_else(|| eyre!("No tag found in file"))?;

    let title = tag
        .title()
        .map(|cow| cow.to_string())
        .or_else(|| {
            path_ref
                .file_stem()
                .map(|os_string| os_string.to_string_lossy().to_string())
        })
        .ok_or_else(|| eyre!("Could not determine a title or file stem for the song"))?;

    let album_title = tag
        .album()
        .map(|cow| cow.to_string())
        .unwrap_or_else(|| "Unknown Album".to_string());

    let artist_name = tag
        .artist()
        .map(|cow| cow.to_string())
        .unwrap_or_else(|| "Unknown Artist".to_string());

    let album_artist_name = tag
        .get_string(ItemKey::AlbumArtist)
        .map(|string| string.to_string())
        .unwrap_or_else(|| artist_name.clone());

    let year = tag
        .get_string(ItemKey::Year)
        .or_else(|| tag.get_string(ItemKey::RecordingDate))
        .or_else(|| tag.get_string(ItemKey::OriginalReleaseDate))
        .and_then(|date_str| {
            let digits: String = date_str
                .chars()
                .filter(|character| character.is_ascii_digit())
                .collect();
            if digits.len() >= 4 {
                digits[..4].parse::<i64>().ok()
            } else {
                date_str.parse::<i64>().ok()
            }
        });

    let track_number = tag.track().map(|number| number as i64);

    let duration = tagged_file.properties().duration().as_secs() as i64;

    let canonical_path = fs::canonicalize(path_ref)
        .await
        .map(|canonical_path_buf| canonical_path_buf.to_string_lossy().to_string())
        .unwrap_or_else(|_| path_ref.to_string_lossy().to_string());

    Ok(ParsedSong {
        title,
        path: canonical_path,
        inode: metadata.ino() as i64,
        mtime: metadata.mtime(),
        size: metadata.len() as i64,
        format,
        album_title,
        year,
        artist_name,
        album_artist_name,
        track_number,
        duration,
    })
}
