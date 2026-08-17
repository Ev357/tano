use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
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

        update_list_area(block.inner(frame.area()));

        let items: Vec<ListItem> = songs
            .displayed()
            .map(|(is_selected, song)| {
                let title = if is_selected {
                    format!("> {}", song.title)
                } else {
                    format!("  {}", song.title)
                };

                ListItem::new(title)
            })
            .collect();

        let list = List::new(items).block(block);

        frame.render_widget(list, frame.area());
    }
}
