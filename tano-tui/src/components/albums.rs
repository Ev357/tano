use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, Paragraph},
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

        let items = albums.to_list_items(|album| album.title.clone());

        let list = List::new(items).block(block);

        frame.render_widget(list, frame.area());
    }
}
