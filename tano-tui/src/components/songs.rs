use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, Paragraph},
};
use tano_database::song::Song;

use crate::utils::{layout_cache::update_list_area, list_state::ListState, load_state::LoadState};

#[derive(Debug, Clone)]
pub struct SongsProps {
    pub songs: LoadState<ListState<Song>>,
}

pub struct SongsComponent {}

impl SongsComponent {
    pub fn render(frame: &mut Frame, props: &SongsProps) {
        let block = Block::bordered()
            .title("Songs")
            .border_type(BorderType::Rounded);

        let songs = match &props.songs {
            LoadState::Loaded(songs) => songs,
            LoadState::Loading => {
                let loading_widget = Paragraph::new("Loading...")
                    .alignment(Alignment::Center)
                    .block(block);

                frame.render_widget(loading_widget, frame.area());
                return;
            }
        };

        let inner_area = block.inner(frame.area());

        update_list_area(inner_area);

        let items = songs.to_list_items(|song| song.title.clone());

        let list = List::new(items).block(block);

        frame.render_widget(list, frame.area());
    }
}
