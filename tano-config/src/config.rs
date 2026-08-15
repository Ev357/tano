use std::sync::LazyLock;

use serde::Deserialize;

use crate::{
    keymaps::keymap::Keymap,
    pages::{Pages, page::Page},
    providers::ProviderConfig,
};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "provider")]
    pub providers: Vec<ProviderConfig>,
    #[serde(rename = "keymap")]
    pub keymaps: Option<Vec<Keymap>>,
    #[serde(rename = "page")]
    pub pages: Pages,
    pub startup_page: Page,
}

static DEFAULT_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let toml_str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/default-config.toml"));
    let config: Config = toml::from_str(toml_str).expect("Invalid default config");

    config
});

impl Default for Config {
    fn default() -> Self {
        DEFAULT_CONFIG.clone()
    }
}
