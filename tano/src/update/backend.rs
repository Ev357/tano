use crossterm::event::{Event, KeyCode};
use tano_backend::actor::msg::BackendMsg;
use tano_tui::actor::msg::TuiMsg;
use tokio::sync::watch;

use crate::{cmd::Cmd, model::Model, msg::Msg};

pub fn update_backend(_model_tx: &watch::Sender<Model>, backend_msg: BackendMsg) -> Cmd {
    match backend_msg {
        BackendMsg::Event(event) => match event {
            Ok(event) => match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => Cmd::Msg(Msg::Restore),
                    _ => Cmd::None,
                },
                Event::Resize(_, _) => Cmd::task(|handles| async move {
                    let result = handles.tui.render().await;
                    Msg::Tui(TuiMsg::RenderDone(result))
                }),
                _ => Cmd::None,
            },
            Err(error) => Cmd::Error(error.into()),
        },
    }
}
