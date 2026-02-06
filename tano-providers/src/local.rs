use tano_config::providers::local::LocalConfig;

#[derive(Debug)]
pub struct LocalProvider {
    pub config: LocalConfig,
}

impl LocalProvider {
    pub fn new(config: LocalConfig) -> Self {
        Self { config }
    }
}
