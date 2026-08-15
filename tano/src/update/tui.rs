use color_eyre::eyre::Result;
use tokio::sync::watch::Sender;

#[derive(Debug)]
pub enum TuiMsg {
    RenderDone(Result<()>),
}

use crate::{cmd::Cmd, model::Model};

pub fn update_tui(_model_tx: &Sender<Model>, tui_msg: TuiMsg) -> Cmd {
    match tui_msg {
        TuiMsg::RenderDone(result) => match result {
            Ok(()) => Cmd::None,
            Err(report) => Cmd::Error(report),
        },
    }
}
