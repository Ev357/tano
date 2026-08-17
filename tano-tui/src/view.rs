use crate::components::{
    album::AlbumProps, albums::AlbumsProps, artists::ArtistsProps, overview::OverviewProps,
    song::SongProps, songs::SongsProps,
};

#[derive(Debug, Default, Clone)]
pub enum View {
    #[default]
    Loading,
    Songs(SongsProps),
    Song(SongProps),
    Album(AlbumProps),
    Albums(AlbumsProps),
    Artists(ArtistsProps),
    Overview(OverviewProps),
}
