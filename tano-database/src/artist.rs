define_entity!(
    CreateArtist,
    Artist {
        id: i64,
        provider_id: i64,
        name: String,
    }
);

#[repr(i64)]
#[derive(Debug, Clone, Copy)]
pub enum ArtistRole {
    Artist = 0,
    AlbumArtist = 1,
}
