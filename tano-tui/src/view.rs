use crate::components::songs::SongsProps;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum View {
    #[default]
    Loading,
    Songs(SongsProps),
}
