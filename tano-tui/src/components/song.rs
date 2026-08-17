use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, Paragraph},
};
use tano_database::{album::Album, artist::Artist, song::Song};

use crate::utils::load_state::LoadState;

#[derive(Debug, Clone)]
pub struct SongProps {
    pub song_id: i64,
    pub data: LoadState<(Song, Option<Album>, Vec<Artist>)>,
}

pub struct SongComponent {}

impl SongComponent {
    pub fn render(frame: &mut Frame, props: &SongProps) {
        let block = Block::bordered().border_type(BorderType::Rounded);

        let (song, album, artists) = match &props.data {
            LoadState::Loaded((song, album, artists)) => (song, album, artists),
            LoadState::Loading => {
                let loading_widget = Paragraph::new("Loading...")
                    .alignment(Alignment::Center)
                    .block(block);

                frame.render_widget(loading_widget, frame.area());
                return;
            }
        };

        let artist_names: Vec<_> = artists.iter().map(|artist| artist.name.clone()).collect();
        let artist_str = if artist_names.is_empty() {
            String::new()
        } else {
            format!(" by {}", artist_names.join(", "))
        };

        let title = format!("{}{}", song.title.as_str(), artist_str);

        let block = block.title(title.as_str());

        let album_name = album
            .as_ref()
            .map(|a| a.title.clone())
            .unwrap_or_else(|| "Unknown Album".to_string());

        let artist_display = if artist_names.is_empty() {
            "Unknown".to_string()
        } else {
            artist_names.join(", ")
        };

        let details = format!(
            "Title: {}\nArtist: {}\nAlbum: {}\nDuration: {} seconds\nTrack: {}",
            song.title,
            artist_display,
            album_name,
            song.duration,
            song.track_number
                .map(|t| t.to_string())
                .unwrap_or_else(|| "-".to_string()),
        );

        let paragraph = Paragraph::new(details)
            .alignment(Alignment::Left)
            .block(block);

        frame.render_widget(paragraph, frame.area());
    }
}
