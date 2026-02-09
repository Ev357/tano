use std::path::PathBuf;

use serde::Deserialize;

use crate::providers::{ProviderConfig, local::LocalConfig};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let expanded_cow = shellexpand::full("~/Music");

        let providers = match expanded_cow {
            Ok(path) => vec![ProviderConfig::Local {
                config: LocalConfig {
                    path: PathBuf::from(path.as_ref()),
                },
            }],
            Err(err) => {
                tracing::warn!(
                    "Couldn't find default local provider directory, using empty providers: {err}"
                );
                vec![]
            }
        };

        Self { providers }
    }
}
