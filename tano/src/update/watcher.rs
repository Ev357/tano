use tano_shared::{get_config_dir::get_config_dir, get_config_file::get_config_file};
use tano_watcher::{actor::msg::WatcherMsg, watch_id::WatchId};
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::{
        Model,
        config_state::{ConfigState, ConfigWatchState},
    },
    msg::Msg,
    update::{config::ConfigMsg, providers::msg::ProvidersMsg},
};

pub fn update_watcher(model_tx: &watch::Sender<Model>, watcher_msg: WatcherMsg) -> Cmd {
    match watcher_msg {
        WatcherMsg::WatchEvent { watch_id, path } => match watch_id {
            WatchId::Config => {
                let should_reload = {
                    let model = model_tx.borrow();

                    if let ConfigState::Loaded { watch_state, .. } = &model.config {
                        if let Ok(config_dir) = get_config_dir() {
                            let config_file = get_config_file(&config_dir);
                            match watch_state {
                                ConfigWatchState::TargetResolved => {
                                    path == config_file || path == config_dir
                                }
                                ConfigWatchState::FallbackConfig => path == config_dir,
                                ConfigWatchState::FallbackHome => {
                                    if let Some(parent) = config_dir.parent() {
                                        path == parent
                                    } else {
                                        false
                                    }
                                }
                                ConfigWatchState::NoHome => false,
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                if !should_reload {
                    return Cmd::None;
                }

                Cmd::task(|handles| async move {
                    let result = handles.config.load_config().await;

                    Msg::Config(ConfigMsg::ConfigLoaded(result))
                })
            }
            WatchId::Provider(provider_id) => {
                Cmd::Msg(Msg::Providers(ProvidersMsg::Sync { provider_id, path }))
            }
        },
        WatcherMsg::Error(report) => Cmd::Error(report),
    }
}
