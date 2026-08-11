use tano_config::providers::local::LocalConfig;

mod get_format;
pub mod parse_song;

#[derive(Debug, Clone)]
pub struct LocalProvider {
    pub config: LocalConfig,
}

impl LocalProvider {
    pub fn new(config: LocalConfig) -> Self {
        Self { config }
    }
}
