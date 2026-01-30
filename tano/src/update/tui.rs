use tano_tui::actor::msg::TuiMsg;
use tokio::sync::watch;

use crate::{cmd::Cmd, model::Model};

pub fn update_tui(_model_tx: &watch::Sender<Model>, tui_msg: TuiMsg) -> Cmd {
    match tui_msg {
        TuiMsg::RenderDone(result) => match result {
            Ok(()) => Cmd::None,
            Err(report) => Cmd::Error(report),
        },
    }
}
