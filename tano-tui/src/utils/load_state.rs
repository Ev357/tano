#[derive(Debug, Clone)]
pub enum LoadState<T> {
    Loaded(T),
    NotLoaded,
}
