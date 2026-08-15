use crate::components::{albums::AlbumsProps, overview::OverviewProps, songs::SongsProps};

#[derive(Debug, Default, Clone)]
pub enum View {
    #[default]
    Loading,
    Songs(SongsProps),
    Albums(AlbumsProps),
    Overview(OverviewProps),
}
