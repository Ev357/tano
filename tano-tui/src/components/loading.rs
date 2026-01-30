use ratatui::{
    Frame,
    layout::Constraint,
    text::Text,
    widgets::{Block, BorderType},
};

pub struct LoadingComponent;

impl LoadingComponent {
    pub fn render(frame: &mut Frame) {
        let block = Block::bordered().border_type(BorderType::Rounded);
        frame.render_widget(block, frame.area());

        let text = Text::raw("Loading...");
        let area = frame.area().centered(
            Constraint::Length(text.width() as u16),
            Constraint::Length(1),
        );
        frame.render_widget(text, area);
    }
}
