use tano_providers::ProviderType;

pub trait WatcherModel: Send + Sync + 'static {
    fn providers(&self) -> &[ProviderType];
}
