use std::collections::{HashMap, HashSet};

use color_eyre::eyre::Result;
use sqlx::{Executor, Sqlite, SqlitePool};
use tano_providers::local::parse_song::ParsedSong;

use crate::{
    album::{Album, CreateAlbum},
    artist::{Artist, ArtistRole, CreateArtist},
    builders::*,
    bulk_builder::BulkBuilder,
    local_song::{CreateLocalSong, LocalSong, SyncLocalSong},
    song::{CreateSong, Song},
};

pub async fn get_songs(executor: impl Executor<'_, Database = Sqlite>) -> Result<Vec<Song>> {
    let songs = sqlx::query_as!(
        Song,
        r#"
        SELECT id, provider_id, album_id, title, track_number, duration, year
        FROM songs
        ORDER BY title
        "#
    )
    .fetch_all(executor)
    .await?;

    Ok(songs)
}

pub async fn get_album_songs(
    executor: impl Executor<'_, Database = Sqlite>,
    album_id: i64,
) -> Result<Vec<Song>> {
    let songs = sqlx::query_as!(
        Song,
        r#"
        SELECT id, provider_id, album_id, title, track_number, duration, year
        FROM songs
        WHERE album_id = ?
        ORDER BY track_number, title
        "#,
        album_id
    )
    .fetch_all(executor)
    .await?;

    Ok(songs)
}

pub async fn get_album_artists(
    executor: impl Executor<'_, Database = Sqlite>,
    album_id: i64,
) -> Result<Vec<Artist>> {
    let artists = sqlx::query_as!(
        Artist,
        r#"
        SELECT DISTINCT artists.id, artists.provider_id, artists.name
        FROM artists
        JOIN song_artists ON artists.id = song_artists.artist_id
        JOIN songs ON song_artists.song_id = songs.id
        WHERE songs.album_id = ? AND song_artists.role = 1
        ORDER BY artists.name
        "#,
        album_id
    )
    .fetch_all(executor)
    .await?;

    Ok(artists)
}

pub async fn get_album(executor: impl Executor<'_, Database = Sqlite>, id: i64) -> Result<Album> {
    let album = sqlx::query_as!(
        Album,
        r#"
        SELECT id, provider_id, title
        FROM albums
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(executor)
    .await?;

    Ok(album)
}

pub async fn get_albums(executor: impl Executor<'_, Database = Sqlite>) -> Result<Vec<Album>> {
    let albums = sqlx::query_as!(
        Album,
        r#"
        SELECT id, provider_id, title
        FROM albums
        ORDER BY title
        "#
    )
    .fetch_all(executor)
    .await?;

    Ok(albums)
}

pub async fn get_artists(executor: impl Executor<'_, Database = Sqlite>) -> Result<Vec<Artist>> {
    let artists = sqlx::query_as!(
        Artist,
        r#"
        SELECT id, provider_id, name
        FROM artists
        ORDER BY name
        "#
    )
    .fetch_all(executor)
    .await?;

    Ok(artists)
}

pub async fn get_song_ids(executor: impl Executor<'_, Database = Sqlite>) -> Result<Vec<i64>> {
    let song_ids: Vec<i64> = sqlx::query_scalar!(
        r#"
        SELECT id
        FROM songs
        ORDER BY title
        "#
    )
    .fetch_all(executor)
    .await?;

    Ok(song_ids)
}

pub async fn get_sync_local_songs(
    executor: impl Executor<'_, Database = Sqlite>,
    provider_id: u64,
) -> Result<Vec<SyncLocalSong>> {
    let records = sqlx::query_as!(
        SyncLocalSong,
        r#"
        SELECT local_songs.song_id, local_songs.path, local_songs.inode, local_songs.mtime, local_songs.size
        FROM local_songs
        JOIN songs ON local_songs.song_id = songs.id
        WHERE songs.provider_id = ?
        "#,
        provider_id as i64
    )
    .fetch_all(executor)
    .await?;

    Ok(records)
}

pub async fn get_local_song_by_path(
    executor: impl Executor<'_, Database = Sqlite>,
    provider_id: u64,
    path: &str,
) -> Result<Option<LocalSong>> {
    let local_song = sqlx::query_as!(
        LocalSong,
        r#"
        SELECT local_songs.song_id, local_songs.path, local_songs.inode, local_songs.mtime, local_songs.size, local_songs.format
        FROM local_songs
        JOIN songs ON local_songs.song_id = songs.id
        WHERE local_songs.path = ? AND songs.provider_id = ?
        LIMIT 1"#,
        path,
        provider_id as i64
    )
    .fetch_optional(executor)
    .await?;

    Ok(local_song)
}

pub async fn get_local_song_by_inode(
    executor: impl Executor<'_, Database = Sqlite>,
    provider_id: u64,
    inode: i64,
) -> Result<Option<LocalSong>> {
    let local_song = sqlx::query_as!(
        LocalSong,
        r#"
        SELECT local_songs.song_id, local_songs.path, local_songs.inode, local_songs.mtime, local_songs.size, local_songs.format
        FROM local_songs
        JOIN songs ON local_songs.song_id = songs.id
        WHERE local_songs.inode = ? AND songs.provider_id = ?
        LIMIT 1"#,
        inode,
        provider_id as i64
    )
    .fetch_optional(executor)
    .await?;

    Ok(local_song)
}

pub async fn insert_song(
    executor: impl Executor<'_, Database = Sqlite>,
    song: &CreateSong,
) -> Result<i64> {
    let record = sqlx::query!(
        r#"
        INSERT INTO songs (provider_id, album_id, title, track_number, duration, year)
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING id AS "id!"
        "#,
        song.provider_id,
        song.album_id,
        song.title,
        song.track_number,
        song.duration,
        song.year
    )
    .fetch_one(executor)
    .await?;

    Ok(record.id)
}

pub async fn insert_local_song(
    executor: impl Executor<'_, Database = Sqlite>,
    song_id: i64,
    local_song: &CreateLocalSong,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO local_songs (song_id, path, inode, mtime, size, format)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        song_id,
        local_song.path,
        local_song.inode,
        local_song.mtime,
        local_song.size,
        local_song.format
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn update_song(
    executor: impl Executor<'_, Database = Sqlite>,
    id: i64,
    song: &CreateSong,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE songs
        SET title = ?, provider_id = ?, album_id = ?, track_number = ?, duration = ?, year = ?
        WHERE id = ?
        "#,
        song.title,
        song.provider_id,
        song.album_id,
        song.track_number,
        song.duration,
        song.year,
        id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn update_local_song(
    executor: impl Executor<'_, Database = Sqlite>,
    id: i64,
    local_song: &CreateLocalSong,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE local_songs
        SET path = ?, inode = ?, mtime = ?, size = ?, format = ?
        WHERE song_id = ?
        "#,
        local_song.path,
        local_song.inode,
        local_song.mtime,
        local_song.size,
        local_song.format,
        id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn update_local_song_path(
    executor: impl Executor<'_, Database = Sqlite>,
    id: i64,
    path: &str,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE local_songs
        SET path = ?
        WHERE song_id = ?
        "#,
        path,
        id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn delete_song(executor: impl Executor<'_, Database = Sqlite>, id: i64) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM songs WHERE id = ?
        "#,
        id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn upsert_album(
    executor: impl Executor<'_, Database = Sqlite>,
    album: &CreateAlbum,
) -> Result<i64> {
    let record = sqlx::query!(
        r#"
        INSERT INTO albums (provider_id, title)
        VALUES (?, ?)
        ON CONFLICT (provider_id, title) DO UPDATE SET title = excluded.title
        RETURNING id AS "id!"
        "#,
        album.provider_id,
        album.title
    )
    .fetch_one(executor)
    .await?;

    Ok(record.id)
}

pub async fn delete_orphan_albums(executor: impl Executor<'_, Database = Sqlite>) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM albums
        WHERE id NOT IN (SELECT album_id FROM songs)
        "#
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn upsert_artist(
    executor: impl Executor<'_, Database = Sqlite>,
    artist: &CreateArtist,
) -> Result<i64> {
    let record = sqlx::query!(
        r#"
        INSERT INTO artists (provider_id, name)
        VALUES (?, ?)
        ON CONFLICT (provider_id, name) DO UPDATE SET name = excluded.name
        RETURNING id AS "id!"
        "#,
        artist.provider_id,
        artist.name
    )
    .fetch_one(executor)
    .await?;

    Ok(record.id)
}

pub async fn delete_orphan_artists(executor: impl Executor<'_, Database = Sqlite>) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM artists
        WHERE id NOT IN (SELECT artist_id FROM song_artists)
        "#
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn insert_song_artist(
    executor: impl Executor<'_, Database = Sqlite>,
    song_id: i64,
    artist_id: i64,
    role: ArtistRole,
) -> Result<()> {
    let role_id = role as i64;
    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO song_artists (song_id, artist_id, role)
        VALUES (?, ?, ?)
        "#,
        song_id,
        artist_id,
        role_id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn delete_song_artists(
    executor: impl Executor<'_, Database = Sqlite>,
    song_id: i64,
) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM song_artists WHERE song_id = ?
        "#,
        song_id
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn sync_local_songs(
    pool: &SqlitePool,
    provider_id: u64,
    new_songs: Vec<ParsedSong>,
    updated_songs: Vec<(i64, ParsedSong)>,
    to_update_path: Vec<(i64, String)>,
    to_delete_ids: Vec<i64>,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    let chunk_size = 1000;

    let mut unique_artists = HashSet::new();
    for parsed in new_songs
        .iter()
        .chain(updated_songs.iter().map(|(_, song)| song))
    {
        unique_artists.insert(parsed.album_artist_name.as_str());
        unique_artists.insert(parsed.artist_name.as_str());
    }

    let provider_id = provider_id as i64;

    let mut artist_map = HashMap::new();
    for chunk in unique_artists.iter().collect::<Vec<_>>().chunks(chunk_size) {
        let mut builder = UpsertArtistsBuilder::new();
        for &&name in chunk {
            let artist = CreateArtist {
                provider_id,
                name: name.to_string(),
            };
            builder.push(&artist);
        }

        let records = builder
            .build()
            .build_query_as::<(i64, String)>()
            .fetch_all(&mut *tx)
            .await?;

        for (id, db_name) in records {
            if let Some(&orig_name) = unique_artists.get(db_name.as_str()) {
                artist_map.insert(orig_name, id);
            }
        }
    }

    let mut unique_albums = HashSet::new();
    for parsed in new_songs
        .iter()
        .chain(updated_songs.iter().map(|(_, song)| song))
    {
        unique_albums.insert(parsed.album_title.as_str());
    }

    let mut album_map = HashMap::new();
    for chunk in unique_albums.iter().collect::<Vec<_>>().chunks(chunk_size) {
        let mut builder = UpsertAlbumsBuilder::new();
        for &&title in chunk {
            let album = CreateAlbum {
                provider_id,
                title: title.to_string(),
            };
            builder.push(&album);
        }

        let records = builder
            .build()
            .build_query_as::<(i64, String)>()
            .fetch_all(&mut *tx)
            .await?;

        for (id, db_title) in records {
            if let Some(&orig_title) = unique_albums.get(db_title.as_str()) {
                album_map.insert(orig_title, id);
            }
        }
    }

    let mut pending_song_artists: Vec<(i64, i64, ArtistRole)> = Vec::new();

    for chunk in new_songs.chunks(chunk_size) {
        let mut builder = InsertSongsBuilder::new();
        for parsed in chunk {
            let album_id = *album_map.get(parsed.album_title.as_str()).unwrap();
            let song = CreateSong {
                provider_id,
                album_id,
                title: parsed.title.clone(),
                track_number: parsed.track_number,
                duration: parsed.duration,
                year: parsed.year,
            };
            builder.push(&song);
        }

        let returned_ids = builder
            .build()
            .build_query_as::<(i64,)>()
            .fetch_all(&mut *tx)
            .await?;

        let mut local_builder = InsertLocalSongsBuilder::new();
        for (parsed, (song_id,)) in chunk.iter().zip(returned_ids.iter()) {
            let local_song = CreateLocalSong {
                path: parsed.path.clone(),
                inode: parsed.inode,
                mtime: parsed.mtime,
                size: parsed.size,
                format: parsed.format.clone(),
            };
            local_builder.push((*song_id, &local_song));

            let album_artist_id = *artist_map.get(parsed.album_artist_name.as_str()).unwrap();
            let track_artist_id = *artist_map.get(parsed.artist_name.as_str()).unwrap();
            pending_song_artists.push((*song_id, album_artist_id, ArtistRole::AlbumArtist));
            pending_song_artists.push((*song_id, track_artist_id, ArtistRole::Artist));
        }
        local_builder.build().build().execute(&mut *tx).await?;
    }

    for chunk in updated_songs.chunks(chunk_size) {
        let mut builder = UpdateSongsBuilder::new();
        for (id, parsed) in chunk {
            let album_id = *album_map.get(parsed.album_title.as_str()).unwrap();
            let song = CreateSong {
                provider_id,
                album_id,
                title: parsed.title.clone(),
                track_number: parsed.track_number,
                duration: parsed.duration,
                year: parsed.year,
            };
            builder.push((*id, &song));
        }
        builder.build().build().execute(&mut *tx).await?;

        let mut local_builder = UpdateLocalSongsBuilder::new();
        for (id, parsed) in chunk {
            let local_song = CreateLocalSong {
                path: parsed.path.clone(),
                inode: parsed.inode,
                mtime: parsed.mtime,
                size: parsed.size,
                format: parsed.format.clone(),
            };
            local_builder.push((*id, &local_song));

            let album_artist_id = *artist_map.get(parsed.album_artist_name.as_str()).unwrap();
            let track_artist_id = *artist_map.get(parsed.artist_name.as_str()).unwrap();
            pending_song_artists.push((*id, album_artist_id, ArtistRole::AlbumArtist));
            pending_song_artists.push((*id, track_artist_id, ArtistRole::Artist));
        }
        local_builder.build().build().execute(&mut *tx).await?;

        let mut delete_builder = DeleteSongArtistsBuilder::new();
        for (id, _) in chunk {
            delete_builder.push(*id);
        }
        delete_builder.build().build().execute(&mut *tx).await?;
    }

    for chunk in pending_song_artists.chunks(chunk_size) {
        let mut builder = InsertSongArtistsBuilder::new();
        for &(song_id, artist_id, role) in chunk {
            builder.push((song_id, artist_id, role));
        }
        builder.build().build().execute(&mut *tx).await?;
    }

    for chunk in to_update_path.chunks(chunk_size) {
        let mut builder = UpdateLocalSongsPathBuilder::new();
        for (id, path) in chunk {
            builder.push((*id, path.as_str()));
        }
        builder.build().build().execute(&mut *tx).await?;
    }

    for chunk in to_delete_ids.chunks(chunk_size) {
        let mut builder = DeleteSongsBuilder::new();
        for &id in chunk {
            builder.push(id);
        }
        builder.build().build().execute(&mut *tx).await?;
    }

    delete_orphan_albums(&mut *tx).await?;
    delete_orphan_artists(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn insert_parsed_song(
    pool: &SqlitePool,
    provider_id: u64,
    parsed: &ParsedSong,
) -> Result<i64> {
    let mut tx = pool.begin().await?;

    let album_id = upsert_album(
        &mut *tx,
        &CreateAlbum {
            provider_id: provider_id as i64,
            title: parsed.album_title.clone(),
        },
    )
    .await?;

    let song_id = insert_song(
        &mut *tx,
        &CreateSong {
            provider_id: provider_id as i64,
            album_id,
            title: parsed.title.clone(),
            track_number: parsed.track_number,
            duration: parsed.duration,
            year: parsed.year,
        },
    )
    .await?;

    let album_artist_id = upsert_artist(
        &mut *tx,
        &CreateArtist {
            provider_id: provider_id as i64,
            name: parsed.album_artist_name.clone(),
        },
    )
    .await?;
    insert_song_artist(&mut *tx, song_id, album_artist_id, ArtistRole::AlbumArtist).await?;

    insert_local_song(
        &mut *tx,
        song_id,
        &CreateLocalSong {
            path: parsed.path.clone(),
            inode: parsed.inode,
            mtime: parsed.mtime,
            size: parsed.size,
            format: parsed.format.clone(),
        },
    )
    .await?;

    let artist_id = upsert_artist(
        &mut *tx,
        &CreateArtist {
            provider_id: provider_id as i64,
            name: parsed.artist_name.clone(),
        },
    )
    .await?;
    insert_song_artist(&mut *tx, song_id, artist_id, ArtistRole::Artist).await?;

    tx.commit().await?;

    Ok(song_id)
}

pub async fn update_parsed_song(
    pool: &SqlitePool,
    provider_id: u64,
    id: i64,
    parsed: &ParsedSong,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    let album_id = upsert_album(
        &mut *tx,
        &CreateAlbum {
            provider_id: provider_id as i64,
            title: parsed.album_title.clone(),
        },
    )
    .await?;

    update_song(
        &mut *tx,
        id,
        &CreateSong {
            provider_id: provider_id as i64,
            album_id,
            title: parsed.title.clone(),
            track_number: parsed.track_number,
            duration: parsed.duration,
            year: parsed.year,
        },
    )
    .await?;

    update_local_song(
        &mut *tx,
        id,
        &CreateLocalSong {
            path: parsed.path.clone(),
            inode: parsed.inode,
            mtime: parsed.mtime,
            size: parsed.size,
            format: parsed.format.clone(),
        },
    )
    .await?;

    delete_song_artists(&mut *tx, id).await?;

    let album_artist_id = upsert_artist(
        &mut *tx,
        &CreateArtist {
            provider_id: provider_id as i64,
            name: parsed.album_artist_name.clone(),
        },
    )
    .await?;
    insert_song_artist(&mut *tx, id, album_artist_id, ArtistRole::AlbumArtist).await?;

    let artist_id = upsert_artist(
        &mut *tx,
        &CreateArtist {
            provider_id: provider_id as i64,
            name: parsed.artist_name.clone(),
        },
    )
    .await?;
    insert_song_artist(&mut *tx, id, artist_id, ArtistRole::Artist).await?;

    delete_orphan_albums(&mut *tx).await?;
    delete_orphan_artists(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn delete_parsed_song(pool: &SqlitePool, id: i64) -> Result<()> {
    let mut tx = pool.begin().await?;

    delete_song(&mut *tx, id).await?;
    delete_orphan_albums(&mut *tx).await?;
    delete_orphan_artists(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}
