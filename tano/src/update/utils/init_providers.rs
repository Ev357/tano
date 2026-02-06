use tano_config::providers::ProviderConfig;
use tano_providers::{ProviderType, local::LocalProvider};

use crate::model::Model;

pub fn init_providers(model: &mut Model, config: Vec<ProviderConfig>) {
    let providers: Vec<ProviderType> = config
        .into_iter()
        .map(|provider_config| match provider_config {
            ProviderConfig::Local { config } => ProviderType::Local(LocalProvider::new(config)),
        })
        .collect();

    model.providers = providers;
}
