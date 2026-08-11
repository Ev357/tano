use tano_config::providers::ProviderConfig;

use crate::local::LocalProvider;

pub mod local;

#[derive(Debug)]
pub enum ProviderType {
    Local(LocalProvider),
}

impl From<ProviderConfig> for ProviderType {
    fn from(provider_config: ProviderConfig) -> Self {
        match provider_config {
            ProviderConfig::Local { config } => ProviderType::Local(LocalProvider::new(config)),
        }
    }
}
