use ratatui::Frame;

use crate::{
    components::{loading::LoadingComponent, songs::SongsComponent},
    view::View,
};

pub struct RootComponent {}

impl RootComponent {
    pub fn render(frame: &mut Frame, props: &View) {
        match props {
            View::Loading => LoadingComponent::render(frame),
            View::Songs(songs) => SongsComponent::render(frame, songs),
        }
    }
}
