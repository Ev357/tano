use ratatui::{
    Frame,
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
        let available_height = frame.area().height;

        let items: Vec<ListItem> = props
            .sections
            .displayed(available_height)
            .map(|(is_selected, page)| {
                let title = if is_selected {
                    format!("> {}", page)
                } else {
                    format!("  {}", page)
                };

                ListItem::new(title)
            })
            .collect();

        let list = List::new(items);

        frame.render_widget(list, frame.area());
    }
}
