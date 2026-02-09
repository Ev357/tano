use color_eyre::eyre::Result;
use tano_backend::actor::msg::BackendMsg;
use tano_config::{actor::msg::ConfigMsg, config::Config, providers::ProviderConfig};
use tano_database::actor::mgs::DatabaseMsg;
use tano_tui::actor::msg::TuiMsg;
use tano_watcher::actor::msg::WatcherMsg;

use crate::update::providers::msg::ProvidersMsg;

#[derive(Debug)]
pub enum Msg {
    Init,
    InitDone { result: Result<Config> },
    InitProviders { config: Vec<ProviderConfig> },
    Restore,
    Close { restore_result: Result<()> },
    Database(DatabaseMsg),
    Backend(BackendMsg),
    Tui(TuiMsg),
    Watcher(WatcherMsg),
    Config(ConfigMsg),
    Providers(ProvidersMsg),
}
