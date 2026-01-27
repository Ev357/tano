use ratatui::Frame;

pub trait Component<T = ()> {
    fn render(&mut self, frame: &mut Frame);
    fn rerender(&mut self, frame: &mut Frame, props: &T);
}
