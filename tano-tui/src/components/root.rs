use ratatui::Frame;

use crate::{
    components::{component::Component, loading::LoadingComponent, songs::SongsComponent},
    view::View,
};

enum RootChild {
    Loading(LoadingComponent),
    Songs(SongsComponent),
}

impl Default for RootChild {
    fn default() -> Self {
        Self::Loading(LoadingComponent)
    }
}

#[derive(Default)]
pub struct RootComponent {
    props: View,
    child: RootChild,
}

impl Component<View> for RootComponent {
    fn render(&mut self, frame: &mut Frame) {
        match &self.props {
            View::Loading => {
                self.child = RootChild::Loading(LoadingComponent);
            }
            View::Songs(songs) => {
                self.child = RootChild::Songs(SongsComponent::new(songs));
            }
        }

        match self.child {
            RootChild::Loading(ref mut loading) => loading.render(frame),
            RootChild::Songs(ref mut songs) => songs.render(frame),
        }
    }

    fn rerender(&mut self, frame: &mut Frame, props: &View) {
        match (props, &mut self.child) {
            (View::Loading, RootChild::Loading(component)) => {
                component.rerender(frame, &());
            }
            (View::Songs(props), RootChild::Songs(component)) => {
                component.rerender(frame, props);
            }
            _ => {
                self.props = props.clone();
                self.render(frame);
            }
        }
    }
}
