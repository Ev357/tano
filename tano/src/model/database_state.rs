#[derive(Debug, Default)]
pub enum DatabaseState {
    #[default]
    Loading,
    Loaded,
}
