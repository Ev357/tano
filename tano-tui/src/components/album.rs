use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, List, Paragraph},
};
use tano_config::pages::album::AlbumPage;
use tano_database::{album::Album, artist::Artist, song::Song};

use crate::utils::{layout_cache::update_list_area, list_state::ListState, load_state::LoadState};

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

        update_list_area(block.inner(frame.area()));

        let items = songs.to_list_items(|song| {
            let track_prefix = if props.config.hide_track_numbers {
                String::new()
            } else {
                match song.track_number {
                    Some(track_number) => format!("{:02} - ", track_number),
                    None => String::new(),
                }
            };

            format!("{}{}", track_prefix, song.title)
        });

        let list = List::new(items).block(block);

        frame.render_widget(list, frame.area());
    }
}
