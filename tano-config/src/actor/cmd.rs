use color_eyre::eyre::Result;
use tokio::sync::oneshot;

use crate::config::Config;

pub enum ConfigCmd {
    LoadConfig {
        respond_to: oneshot::Sender<Result<Config>>,
    },
}
