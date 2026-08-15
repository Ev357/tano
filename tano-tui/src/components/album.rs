use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};
use tano_config::pages::album::AlbumPage;
use tano_database::{album::Album, artist::Artist, song::Song};

use crate::utils::{list_state::ListState, load_state::LoadState};

#[derive(Debug, Clone)]
pub struct AlbumProps {
    pub album_id: i64,
    pub config: AlbumPage,
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

        let artist_names: Vec<_> = artists.iter().map(|artist| artist.name.clone()).collect();
        let artist_str = match (props.config.hide_artists, artist_names.is_empty()) {
            (false, false) => format!(" by {}", artist_names.join(", ")),
            _ => String::new(),
        };

        let title = format!("{}{}", album.title.as_str(), artist_str);

        let block = block.title(title.as_str());

        let items: Vec<ListItem> = songs
            .items
            .iter()
            .enumerate()
            .map(|(index, song)| {
                let track_prefix = match (props.config.hide_track_numbers, song.track_number) {
                    (false, Some(track_number)) => format!("{:02} - ", track_number),
                    _ => String::new(),
                };

                let title = if songs.selected_index == Some(index) {
                    format!("> {}{}", track_prefix, song.title)
                } else {
                    format!("  {}{}", track_prefix, song.title)
                };

                ListItem::new(title)
            })
            .collect();

        let list = List::new(items).block(block);

        frame.render_widget(list, frame.area());
    }
}
