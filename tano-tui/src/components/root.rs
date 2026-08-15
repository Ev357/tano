use ratatui::Frame;

use crate::{
    components::{
        albums::AlbumsComponent, artists::ArtistsComponent, loading::LoadingComponent,
        overview::OverviewComponent, songs::SongsComponent,
    },
    view::View,
};

pub struct RootComponent {}

impl RootComponent {
    pub fn render(frame: &mut Frame, props: &View) {
        match props {
            View::Loading => LoadingComponent::render(frame),
            View::Songs(songs) => SongsComponent::render(frame, songs),
            View::Albums(albums) => AlbumsComponent::render(frame, albums),
            View::Artists(artists) => ArtistsComponent::render(frame, artists),
            View::Overview(pages) => OverviewComponent::render(frame, pages),
        }
    }
}
