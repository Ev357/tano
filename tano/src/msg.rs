use color_eyre::eyre::Result;
use tano_backend::actor::msg::BackendMsg;
use tano_config::{actor::msg::ConfigMsg, config::Config};
use tano_database::actor::mgs::DatabaseMsg;
use tano_tui::actor::msg::TuiMsg;
use tano_watcher::actor::msg::WatcherMsg;

#[derive(Debug)]
pub enum Msg {
    Init,
    InitDone { result: Result<Config> },
    Restore,
    Close { restore_result: Result<()> },
    Database(DatabaseMsg),
    Backend(BackendMsg),
    Tui(TuiMsg),
    Watcher(WatcherMsg),
    Config(ConfigMsg),
}
