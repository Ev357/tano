use crate::components::{
    album::AlbumProps, albums::AlbumsProps, artists::ArtistsProps, overview::OverviewProps,
    songs::SongsProps,
};

#[derive(Debug, Default, Clone)]
pub enum View {
    #[default]
    Loading,
    Songs(SongsProps),
    Album(AlbumProps),
    Albums(AlbumsProps),
    Artists(ArtistsProps),
    Overview(OverviewProps),
}
