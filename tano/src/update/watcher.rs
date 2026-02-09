use tano_config::actor::msg::ConfigMsg;
use tano_watcher::{
    actor::msg::WatcherMsg,
    watch_type::{WatchProvider, WatchType},
};
use tokio::sync::watch;

use crate::{cmd::Cmd, model::Model, msg::Msg, update::providers::msg::ProvidersMsg};

pub fn update_watcher(_model_tx: &watch::Sender<Model>, watcher_msg: WatcherMsg) -> Cmd {
    match watcher_msg {
        WatcherMsg::FileChange {
            watch_type,
            path: _,
        } => match watch_type {
            WatchType::Config => Cmd::task(|handles| async move {
                let result = handles.config.load_config().await;

                Msg::Config(ConfigMsg::ConfigLoaded(result))
            }),
            WatchType::Provider(watch_provider) => match watch_provider {
                WatchProvider::Local => Cmd::Msg(Msg::Providers(ProvidersMsg::Sync)),
            },
        },
    }
}
