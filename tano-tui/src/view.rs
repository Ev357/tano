use crate::components::{overview::OverviewProps, songs::SongsProps};

#[derive(Debug, Default, Clone)]
pub enum View {
    #[default]
    Loading,
    Songs(SongsProps),
    Overview(OverviewProps),
}
