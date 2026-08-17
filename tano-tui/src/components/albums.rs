use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};
use tano_database::album::Album;

use crate::utils::{layout_cache::update_list_area, list_state::ListState, load_state::LoadState};

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

        update_list_area(block.inner(frame.area()));

        let items: Vec<ListItem> = albums
            .displayed()
            .map(|(is_selected, album)| {
                let title = if is_selected {
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
