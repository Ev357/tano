use tano_config::config::Config;

#[derive(Default, Debug)]
pub enum ConfigState {
    #[default]
    Loading,
    Loaded(Config),
}
