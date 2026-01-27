use serde::Deserialize;

use crate::providers::local::LocalConfig;

pub mod local;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ProviderConfig {
    #[serde(rename = "local")]
    Local {
        #[serde(flatten)]
        config: LocalConfig,
    },
}
