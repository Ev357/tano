use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};
use tano_database::{album::Album, artist::Artist, song::Song};

use crate::utils::{list_state::ListState, load_state::LoadState};

#[derive(Debug, Clone)]
pub struct AlbumProps {
    pub album_id: i64,
    pub data: LoadState<(Album, Vec<Artist>, ListState<Song>)>,
}

pub struct AlbumComponent {}

impl AlbumComponent {
    pub fn render(frame: &mut Frame, props: &AlbumProps) {
        let block = Block::bordered().border_type(BorderType::Rounded);

        let (album, artists, songs) = match &props.data {
            LoadState::Loaded((album, artists, songs)) => (album, artists, songs),
            LoadState::Loading => {
                let loading_widget = Paragraph::new("Loading...")
                    .alignment(Alignment::Center)
                    .block(block);

                frame.render_widget(loading_widget, frame.area());
                return;
            }
        };

        let artist_names: Vec<String> = artists.iter().map(|a| a.name.clone()).collect();
        let artist_str = if artist_names.is_empty() {
            String::new()
        } else {
            format!(" by {}", artist_names.join(", "))
        };
        let title = format!("{}{}", album.title.as_str(), artist_str);

        let block = block.title(title.as_str());

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
