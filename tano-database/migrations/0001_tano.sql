CREATE TABLE IF NOT EXISTS artists (
  id INTEGER PRIMARY KEY,
  provider_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  UNIQUE (provider_id, name)
);

CREATE TABLE IF NOT EXISTS albums (
  id INTEGER PRIMARY KEY,
  provider_id INTEGER NOT NULL,
  title TEXT NOT NULL,
  UNIQUE (provider_id, title)
);

CREATE TABLE IF NOT EXISTS songs (
  id INTEGER PRIMARY KEY,
  provider_id INTEGER NOT NULL,
  album_id INTEGER NOT NULL,
  title TEXT NOT NULL,
  track_number INTEGER,
  duration INTEGER NOT NULL,
  year INTEGER,
  FOREIGN KEY (album_id) REFERENCES albums (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS local_songs (
  song_id INTEGER PRIMARY KEY,
  path TEXT NOT NULL,
  inode INTEGER NOT NULL,
  mtime INTEGER NOT NULL,
  size INTEGER NOT NULL,
  format TEXT NOT NULL,
  FOREIGN KEY (song_id) REFERENCES songs (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS song_artists (
  song_id INTEGER NOT NULL,
  artist_id INTEGER NOT NULL,
  role INTEGER NOT NULL,
  PRIMARY KEY (song_id, artist_id, role),
  FOREIGN KEY (song_id) REFERENCES songs (id) ON DELETE CASCADE,
  FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS playlists (
  id INTEGER PRIMARY KEY,
  provider_id INTEGER NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS playlist_songs (
  playlist_id INTEGER NOT NULL,
  song_id INTEGER NOT NULL,
  position INTEGER NOT NULL,
  PRIMARY KEY (playlist_id, position),
  FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE,
  FOREIGN KEY (song_id) REFERENCES songs (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS index_songs_album ON songs (album_id);

CREATE INDEX IF NOT EXISTS index_song_artists_artist ON song_artists (artist_id);

CREATE INDEX IF NOT EXISTS index_playlist_songs_song ON playlist_songs (song_id);
