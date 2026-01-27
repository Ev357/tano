use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem},
};
use tano_database::song::Song;

use crate::components::component::Component;

#[derive(Debug, PartialEq, Clone)]
pub struct SongsProps {
    pub songs: Vec<Song>,
}

pub struct SongsComponent {
    props: SongsProps,
}

impl SongsComponent {
    pub fn new(props: &SongsProps) -> Self {
        Self {
            props: props.clone(),
        }
    }
}

impl Component<SongsProps> for SongsComponent {
    fn render(&mut self, frame: &mut Frame) {
        let items: Vec<ListItem> = self
            .props
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

    fn rerender(&mut self, frame: &mut Frame, props: &SongsProps) {
        if &self.props == props {
            return;
        }

        self.props = props.clone();
        self.render(frame);
    }
}
