use tano_config::actor::msg::ConfigMsg;
use tano_macro::trace_dbg;
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::{
        Model,
        config_state::{ConfigState, ConfigWatchState},
    },
    msg::Msg,
};

pub fn update_config(model_tx: &watch::Sender<Model>, config_msg: ConfigMsg) -> Cmd {
    match config_msg {
        ConfigMsg::ConfigLoaded(config) => match config {
            Ok(config) => {
                trace_dbg!(&config);
                let watch_state = ConfigWatchState::resolve();
                trace_dbg!(&watch_state);
                model_tx.send_modify(|model| model.config = ConfigState::Loaded(watch_state));

                Cmd::Msg(Msg::InitProviders {
                    config: config.providers,
                })
            }
            Err(report) => Cmd::Error(report),
        },
    }
}
