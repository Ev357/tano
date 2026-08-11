use tano_config::actor::msg::ConfigMsg;
use tano_watcher::{actor::msg::WatcherMsg, watch_id::WatchId};
use tokio::sync::watch;

use crate::{cmd::Cmd, model::Model, msg::Msg, update::providers::msg::ProvidersMsg};

pub fn update_watcher(_model_tx: &watch::Sender<Model>, watcher_msg: WatcherMsg) -> Cmd {
    match watcher_msg {
        WatcherMsg::WatchEvent { watch_id, path } => match watch_id {
            WatchId::Config => Cmd::task(|handles| async move {
                let result = handles.config.load_config().await;

                Msg::Config(ConfigMsg::ConfigLoaded(result))
            }),
            WatchId::Provider(provider_id) => {
                Cmd::Msg(Msg::Providers(ProvidersMsg::Sync { provider_id, path }))
            }
        },
    }
}
