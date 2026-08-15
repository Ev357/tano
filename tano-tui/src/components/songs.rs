use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};
use tano_database::song::Song;

use crate::utils::{list_state::ListState, load_state::LoadState};

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

        let items: Vec<ListItem> = songs
            .items
            .iter()
            .enumerate()
            .map(|(index, song)| {
                let title = if songs.selected_index == Some(index) {
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
