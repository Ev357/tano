use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, Paragraph},
};
use tano_database::artist::Artist;

use crate::utils::{layout_cache::update_list_area, list_state::ListState, load_state::LoadState};

#[derive(Debug, Clone)]
pub struct ArtistsProps {
    pub artists: LoadState<ListState<Artist>>,
}

pub struct ArtistsComponent {}

impl ArtistsComponent {
    pub fn render(frame: &mut Frame, props: &ArtistsProps) {
        let block = Block::bordered()
            .title("Artists")
            .border_type(BorderType::Rounded);

        let artists = match &props.artists {
            LoadState::Loaded(artists) => artists,
            LoadState::Loading => {
                let loading_widget = Paragraph::new("Loading...")
                    .alignment(Alignment::Center)
                    .block(block);

                frame.render_widget(loading_widget, frame.area());
                return;
            }
        };

        update_list_area(block.inner(frame.area()));

        let items = artists.to_list_items(|artist| artist.name.clone());

        let list = List::new(items).block(block);

        frame.render_widget(list, frame.area());
    }
}
