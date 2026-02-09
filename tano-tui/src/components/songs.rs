use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem},
};
use tano_database::song::Song;

use crate::utils::list_state::ListState;

#[derive(Debug, PartialEq, Clone)]
pub struct SongsProps {
    pub songs: ListState<Song>,
}

pub struct SongsComponent {}

impl SongsComponent {
    pub fn render(frame: &mut Frame, props: &SongsProps) {
        let items: Vec<ListItem> = props
            .songs
            .items
            .iter()
            .enumerate()
            .map(|(index, song)| {
                let title = if props.songs.selected_index == Some(index) {
                    format!("> {}", song.title)
                } else {
                    format!("  {}", song.title)
                };

                ListItem::new(title)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .title("Songs")
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White));

        frame.render_widget(list, frame.area());
    }
}
