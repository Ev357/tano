use crossterm::event::Event;
use tano_backend::actor::msg::BackendMsg;
use tano_config::action::Action;
use tano_tui::{actor::msg::TuiMsg, view::View};
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::Model,
    msg::Msg,
    update::backend::{handle_keypress::handle_keypress, parse_key_event::parse_key_event},
};

mod handle_keypress;
pub mod parse_key_event;

pub fn update_backend(model_tx: &watch::Sender<Model>, backend_msg: BackendMsg) -> Cmd {
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

                    match triggered_action {
                        Some(Action::Quit) => Cmd::Msg(Msg::Restore),
                        Some(action @ (Action::Next | Action::Previous)) => {
                            model_tx.send_modify(|model| {
                                let songs_props = match &mut model.view {
                                    View::Songs(songs_props) => songs_props,
                                    _ => return,
                                };

                                match action {
                                    Action::Next => songs_props.songs.next(),
                                    Action::Previous => songs_props.songs.previous(),
                                    _ => unreachable!(),
                                }
                            });

                            Cmd::task(|handles| async move {
                                let result = handles.tui.render().await;
                                Msg::Tui(TuiMsg::RenderDone(result))
                            })
                        }
                        None => Cmd::None,
                    }
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
