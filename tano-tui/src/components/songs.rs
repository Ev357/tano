use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem},
};
use tano_database::song::Song;

#[derive(Debug, PartialEq, Clone)]
pub struct SongsProps {
    pub songs: Vec<Song>,
}

pub struct SongsComponent {}

impl SongsComponent {
    pub fn render(frame: &mut Frame, props: &SongsProps) {
        let items: Vec<ListItem> = props
            .songs
            .iter()
            .map(|song| ListItem::new(song.title.clone()))
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
