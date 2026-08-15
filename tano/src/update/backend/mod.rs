use crossterm::event::Event;
use tano_backend::actor::msg::BackendMsg;
use tokio::sync::watch::Sender;

use crate::{
    cmd::Cmd,
    model::Model,
    msg::Msg,
    update::{
        backend::{
            handle_action::handle_action, handle_keypress::handle_keypress,
            parse_key_event::parse_key_event,
        },
        tui::TuiMsg,
    },
};

mod handle_action;
mod handle_keypress;
pub mod parse_key_event;

pub fn update_backend(model_tx: &Sender<Model>, backend_msg: BackendMsg) -> Cmd {
    match backend_msg {
        BackendMsg::Event(event) => match event {
            Ok(event) => match event {
                Event::Key(key_event) => {
                    let keybind = match parse_key_event(&key_event) {
                        Some(keybind) => keybind,
                        None => return Cmd::None,
                    };

                    let mut triggered_action = None;
                    model_tx.send_modify(|model| {
                        triggered_action = handle_keypress(model, keybind);
                    });

                    let action = match triggered_action {
                        Some(action) => action,
                        None => return Cmd::None,
                    };

                    handle_action(model_tx, &action)
                }
                Event::Resize(_, _) => Cmd::task(|handles| async move {
                    let result = handles.tui.render().await;
                    Msg::Tui(TuiMsg::RenderDone(result))
                }),
                _ => Cmd::None,
            },
            Err(error) => Cmd::Error(error.into()),
        },
        BackendMsg::Error(report) => Cmd::Error(report),
    }
}
