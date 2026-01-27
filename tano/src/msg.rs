use color_eyre::eyre::Result;
use tano_backend::actor::msg::BackendMsg;
use tano_config::config::Config;
use tano_database::actor::mgs::DatabaseMsg;

#[derive(Debug)]
pub enum Msg {
    Init,
    InitDone { result: Result<Config> },
    Database(DatabaseMsg),
    Restore,
    Close { restore_result: Result<()> },
    Backend(BackendMsg),
}
