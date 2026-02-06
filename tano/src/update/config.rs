use tano_config::actor::msg::ConfigMsg;
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::{Model, config_state::ConfigState},
    msg::Msg,
};

pub fn update_config(model_tx: &watch::Sender<Model>, config_msg: ConfigMsg) -> Cmd {
    match config_msg {
        ConfigMsg::ConfigLoaded(config) => match config {
            Ok(config) => {
                model_tx.send_modify(|model| model.config = ConfigState::Loaded);

                Cmd::Msg(Msg::InitProviders {
                    config: config.providers,
                })
            }
            Err(report) => Cmd::Error(report),
        },
    }
}
