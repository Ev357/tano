use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{List, ListItem},
};
use tano_config::pages::page::Page;

use crate::utils::list_state::ListState;

#[derive(Debug, Clone)]
pub struct OverviewProps {
    pub sections: ListState<Page>,
}

pub struct OverviewComponent;

impl OverviewComponent {
    pub fn render(frame: &mut Frame, props: &OverviewProps) {
        let items: Vec<ListItem> = props
            .sections
            .items
            .iter()
            .enumerate()
            .map(|(index, song)| {
                let title = if props.sections.selected_index == Some(index) {
                    format!("> {}", song)
                } else {
                    format!("  {}", song)
                };

                ListItem::new(title)
            })
            .collect();

        let list = List::new(items).style(Style::default().fg(Color::White));

        frame.render_widget(list, frame.area());
    }
}
