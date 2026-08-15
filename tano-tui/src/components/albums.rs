use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};
use tano_database::album::Album;

use crate::utils::{list_state::ListState, load_state::LoadState};

#[derive(Debug, Clone)]
pub struct AlbumsProps {
    pub albums: LoadState<ListState<Album>>,
}

pub struct AlbumsComponent {}

impl AlbumsComponent {
    pub fn render(frame: &mut Frame, props: &AlbumsProps) {
        let block = Block::bordered()
            .title("Albums")
            .border_type(BorderType::Rounded);

        let albums = match &props.albums {
            LoadState::Loaded(albums) => albums,
            LoadState::Loading => {
                let loading_widget = Paragraph::new("Loading...")
                    .alignment(Alignment::Center)
                    .block(block);

                frame.render_widget(loading_widget, frame.area());
                return;
            }
        };

        let items: Vec<ListItem> = albums
            .items
            .iter()
            .enumerate()
            .map(|(index, album)| {
                let title = if albums.selected_index == Some(index) {
                    format!("> {}", album.title)
                } else {
                    format!("  {}", album.title)
                };

                ListItem::new(title)
            })
            .collect();

        let list = List::new(items).block(block);

        frame.render_widget(list, frame.area());
    }
}
