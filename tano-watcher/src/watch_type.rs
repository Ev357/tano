#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WatchType {
    Config,
    Provider(WatchProvider),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WatchProvider {
    Local,
}
