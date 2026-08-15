use crate::components::{
    albums::AlbumsProps, artists::ArtistsProps, overview::OverviewProps, songs::SongsProps,
};

#[derive(Debug, Default, Clone)]
pub enum View {
    #[default]
    Loading,
    Songs(SongsProps),
    Albums(AlbumsProps),
    Artists(ArtistsProps),
    Overview(OverviewProps),
}
