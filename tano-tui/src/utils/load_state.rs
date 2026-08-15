#[derive(Debug, Clone, Default)]
pub enum LoadState<T> {
    #[default]
    Loading,
    Loaded(T),
}
