define_entity!(CreateSong, Song {
    id: i64,
    provider_id: i64,
    album_id: i64,
    title: String,
    track_number: Option<i64>,
    duration: i64,
    year: Option<i64>,
});
