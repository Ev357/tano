use ratatui::{Frame, widgets::List};
use tano_config::pages::page::Page;

use crate::utils::{layout_cache::update_list_area, list_state::ListState};

#[derive(Debug, Clone)]
pub struct OverviewProps {
    pub sections: ListState<Page>,
}

pub struct OverviewComponent;

impl OverviewComponent {
    pub fn render(frame: &mut Frame, props: &OverviewProps) {
        update_list_area(frame.area());

        let items = props.sections.to_list_items(|page| page.to_string());

        let list = List::new(items);

        frame.render_widget(list, frame.area());
    }
}
