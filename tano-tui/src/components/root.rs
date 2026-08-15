use ratatui::Frame;

use crate::{
    components::{
        albums::AlbumsComponent, loading::LoadingComponent, overview::OverviewComponent,
        songs::SongsComponent,
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
            View::Overview(pages) => OverviewComponent::render(frame, pages),
        }
    }
}
