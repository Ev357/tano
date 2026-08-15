use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};
use tano_database::artist::Artist;

use crate::utils::{list_state::ListState, load_state::LoadState};

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
            LoadState::NotLoaded => {
                let loading_widget = Paragraph::new("Loading...")
                    .alignment(Alignment::Center)
                    .block(block);

                frame.render_widget(loading_widget, frame.area());
                return;
            }
        };

        let items: Vec<ListItem> = artists
            .items
            .iter()
            .enumerate()
            .map(|(index, artist)| {
                let name = if artists.selected_index == Some(index) {
                    format!("> {}", artist.name)
                } else {
                    format!("  {}", artist.name)
                };

                ListItem::new(name)
            })
            .collect();

        let list = List::new(items).block(block);

        frame.render_widget(list, frame.area());
    }
}
