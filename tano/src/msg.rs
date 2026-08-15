use color_eyre::eyre::Result;
use tano_backend::actor::msg::BackendMsg;
use tano_config::{config::Config, pages::page::Page, providers::ProviderConfig};
use tano_watcher::actor::msg::WatcherMsg;

use crate::update::{
    config::ConfigMsg, database::DatabaseMsg, providers::msg::ProvidersMsg, tui::TuiMsg,
};

#[derive(Debug)]
pub enum Msg {
    Init,
    CoreDataLoaded { result: Result<Config> },
    InitProviders { config: Vec<ProviderConfig> },
    Restore,
    Close { restore_result: Result<()> },
    Database(DatabaseMsg),
    Backend(BackendMsg),
    Tui(TuiMsg),
    Watcher(WatcherMsg),
    Config(ConfigMsg),
    Providers(ProvidersMsg),
    InitInitialView { startup_page: Page },
    Navigate(Page),
    RefreshView,
}
