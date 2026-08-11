#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WatchId {
    Config,
    Provider(u64),
}
