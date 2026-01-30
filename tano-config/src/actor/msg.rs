use color_eyre::eyre::Result;

use crate::config::Config;

#[derive(Debug)]
pub enum ConfigMsg {
    ConfigLoaded(Result<Config>),
}
