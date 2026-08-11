use sqlx::{QueryBuilder, Sqlite};

use crate::{
    album::CreateAlbum,
    artist::{ArtistRole, CreateArtist},
    bulk_builder::BulkBuilder,
    local_song::CreateLocalSong,
    song::CreateSong,
};

pub struct UpsertArtistsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl UpsertArtistsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new("INSERT INTO artists (provider_id, name) VALUES "),
            has_items: false,
        }
    }
}

impl Default for UpsertArtistsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for UpsertArtistsBuilder {
    type Item<'a> = &'a CreateArtist;

    fn push(&mut self, artist: Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb
            .push("(")
            .push_bind(artist.provider_id)
            .push(", ")
            .push_bind(&artist.name)
            .push(")");
    }

    fn build(mut self) -> QueryBuilder<Sqlite> {
        self.qb.push(" ON CONFLICT (provider_id, name) DO UPDATE SET name = excluded.name RETURNING id, name");
        self.qb
    }
}

pub struct UpsertAlbumsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl UpsertAlbumsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new("INSERT INTO albums (provider_id, title) VALUES "),
            has_items: false,
        }
    }
}

impl Default for UpsertAlbumsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for UpsertAlbumsBuilder {
    type Item<'a> = &'a CreateAlbum;

    fn push(&mut self, album: Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb
            .push("(")
            .push_bind(album.provider_id)
            .push(", ")
            .push_bind(&album.title)
            .push(")");
    }

    fn build(mut self) -> QueryBuilder<Sqlite> {
        self.qb.push(" ON CONFLICT (provider_id, title) DO UPDATE SET title = excluded.title RETURNING id, title");
        self.qb
    }
}

pub struct InsertSongsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl InsertSongsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new(
                "INSERT INTO songs (provider_id, album_id, title, track_number, duration, year) VALUES ",
            ),
            has_items: false,
        }
    }
}

impl Default for InsertSongsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for InsertSongsBuilder {
    type Item<'a> = &'a CreateSong;

    fn push(&mut self, song: Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb
            .push("(")
            .push_bind(song.provider_id)
            .push(", ")
            .push_bind(song.album_id)
            .push(", ")
            .push_bind(&song.title)
            .push(", ")
            .push_bind(song.track_number)
            .push(", ")
            .push_bind(song.duration)
            .push(", ")
            .push_bind(song.year)
            .push(")");
    }

    fn build(mut self) -> QueryBuilder<Sqlite> {
        self.qb.push(" RETURNING id");
        self.qb
    }
}

pub struct InsertLocalSongsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl InsertLocalSongsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new(
                "INSERT INTO local_songs (song_id, path, inode, mtime, size, format) VALUES ",
            ),
            has_items: false,
        }
    }
}

impl Default for InsertLocalSongsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for InsertLocalSongsBuilder {
    type Item<'a> = (i64, &'a CreateLocalSong);

    fn push(&mut self, (song_id, song): Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb
            .push("(")
            .push_bind(song_id)
            .push(", ")
            .push_bind(&song.path)
            .push(", ")
            .push_bind(song.inode)
            .push(", ")
            .push_bind(song.mtime)
            .push(", ")
            .push_bind(song.size)
            .push(", ")
            .push_bind(&song.format)
            .push(")");
    }

    fn build(self) -> QueryBuilder<Sqlite> {
        self.qb
    }
}

pub struct UpdateSongsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl UpdateSongsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new(
                "WITH tmp(id, album_id, title, track_number, duration, year) AS (VALUES ",
            ),
            has_items: false,
        }
    }
}

impl Default for UpdateSongsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for UpdateSongsBuilder {
    type Item<'a> = (i64, &'a CreateSong);

    fn push(&mut self, (id, song): Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb
            .push("(")
            .push_bind(id)
            .push(", ")
            .push_bind(song.album_id)
            .push(", ")
            .push_bind(&song.title)
            .push(", ")
            .push_bind(song.track_number)
            .push(", ")
            .push_bind(song.duration)
            .push(", ")
            .push_bind(song.year)
            .push(")");
    }

    fn build(mut self) -> QueryBuilder<Sqlite> {
        self.qb
            .push(") UPDATE songs SET album_id = tmp.album_id, title = tmp.title, track_number = tmp.track_number, duration = tmp.duration, year = tmp.year FROM tmp WHERE songs.id = tmp.id");
        self.qb
    }
}

pub struct UpdateLocalSongsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl UpdateLocalSongsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new("WITH tmp(id, inode, mtime, size, format) AS (VALUES "),
            has_items: false,
        }
    }
}

impl Default for UpdateLocalSongsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for UpdateLocalSongsBuilder {
    type Item<'a> = (i64, &'a CreateLocalSong);

    fn push(&mut self, (id, song): Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb
            .push("(")
            .push_bind(id)
            .push(", ")
            .push_bind(song.inode)
            .push(", ")
            .push_bind(song.mtime)
            .push(", ")
            .push_bind(song.size)
            .push(", ")
            .push_bind(&song.format)
            .push(")");
    }

    fn build(mut self) -> QueryBuilder<Sqlite> {
        self.qb
            .push(") UPDATE local_songs SET inode = tmp.inode, mtime = tmp.mtime, size = tmp.size, format = tmp.format FROM tmp WHERE local_songs.song_id = tmp.id");
        self.qb
    }
}

pub struct InsertSongArtistsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl InsertSongArtistsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new(
                "INSERT OR IGNORE INTO song_artists (song_id, artist_id, role) VALUES ",
            ),
            has_items: false,
        }
    }
}

impl Default for InsertSongArtistsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for InsertSongArtistsBuilder {
    type Item<'a> = (i64, i64, ArtistRole);

    fn push(&mut self, (song_id, artist_id, role): Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb
            .push("(")
            .push_bind(song_id)
            .push(", ")
            .push_bind(artist_id)
            .push(", ")
            .push_bind(role as i64)
            .push(")");
    }

    fn build(self) -> QueryBuilder<Sqlite> {
        self.qb
    }
}

pub struct UpdateLocalSongsPathBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl UpdateLocalSongsPathBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new("WITH tmp(id, path) AS (VALUES "),
            has_items: false,
        }
    }
}

impl Default for UpdateLocalSongsPathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for UpdateLocalSongsPathBuilder {
    type Item<'a> = (i64, &'a str);

    fn push(&mut self, (id, path): Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb
            .push("(")
            .push_bind(id)
            .push(", ")
            .push_bind(path)
            .push(")");
    }

    fn build(mut self) -> QueryBuilder<Sqlite> {
        self.qb.push(
            ") UPDATE local_songs SET path = tmp.path FROM tmp WHERE local_songs.song_id = tmp.id",
        );
        self.qb
    }
}

pub struct DeleteSongArtistsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl DeleteSongArtistsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new("DELETE FROM song_artists WHERE song_id IN ("),
            has_items: false,
        }
    }
}

impl Default for DeleteSongArtistsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for DeleteSongArtistsBuilder {
    type Item<'a> = i64;

    fn push(&mut self, id: Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb.push_bind(id);
    }

    fn build(mut self) -> QueryBuilder<Sqlite> {
        self.qb.push(")");
        self.qb
    }
}

pub struct DeleteSongsBuilder {
    qb: QueryBuilder<Sqlite>,
    has_items: bool,
}

impl DeleteSongsBuilder {
    pub fn new() -> Self {
        Self {
            qb: QueryBuilder::new("DELETE FROM songs WHERE id IN ("),
            has_items: false,
        }
    }
}

impl Default for DeleteSongsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BulkBuilder for DeleteSongsBuilder {
    type Item<'a> = i64;

    fn push(&mut self, id: Self::Item<'_>) {
        if self.has_items {
            self.qb.push(", ");
        } else {
            self.has_items = true;
        }
        self.qb.push_bind(id);
    }

    fn build(mut self) -> QueryBuilder<Sqlite> {
        self.qb.push(")");
        self.qb
    }
}
