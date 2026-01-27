use std::path::PathBuf;

use serde::Deserialize;

use crate::providers::{ProviderConfig, local::LocalConfig};

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default = "default_providers")]
    pub providers: Vec<ProviderConfig>,
}

fn default_providers() -> Vec<ProviderConfig> {
    vec![ProviderConfig::Local {
        config: LocalConfig {
            path: PathBuf::from("~/Music"),
        },
    }]
}
