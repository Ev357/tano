use crossterm::event::{Event, MouseEvent, MouseEventKind};
use tano_backend::actor::msg::BackendMsg;
use tano_tui::{utils::load_state::LoadState, view::View};
use tokio::sync::watch::Sender;

use crate::{
    cmd::Cmd,
    model::Model,
    msg::Msg,
    update::{
        backend::{
            handle_action::{
                handle_action, handle_load_state_scroll_down, handle_load_state_scroll_up,
            },
            handle_keypress::handle_keypress,
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
                Event::Mouse(MouseEvent { kind, .. }) => {
                    let modified = model_tx.send_if_modified(|model| match &mut model.view {
                        View::Songs(props) => match kind {
                            MouseEventKind::ScrollDown => {
                                handle_load_state_scroll_down(&mut props.songs)
                            }
                            MouseEventKind::ScrollUp => {
                                handle_load_state_scroll_up(&mut props.songs)
                            }
                            _ => false,
                        },
                        View::Album(props) => {
                            let list = match &mut props.data {
                                LoadState::Loaded((_, _, list)) => list,

                                LoadState::Loading => return false,
                            };

                            match kind {
                                MouseEventKind::ScrollDown => {
                                    list.scroll_down();
                                    true
                                }
                                MouseEventKind::ScrollUp => {
                                    list.scroll_up();
                                    true
                                }
                                _ => false,
                            }
                        }
                        View::Albums(props) => match kind {
                            MouseEventKind::ScrollDown => {
                                handle_load_state_scroll_down(&mut props.albums)
                            }
                            MouseEventKind::ScrollUp => {
                                handle_load_state_scroll_up(&mut props.albums)
                            }
                            _ => false,
                        },
                        View::Artists(props) => match kind {
                            MouseEventKind::ScrollDown => {
                                handle_load_state_scroll_down(&mut props.artists)
                            }
                            MouseEventKind::ScrollUp => {
                                handle_load_state_scroll_up(&mut props.artists)
                            }
                            _ => false,
                        },
                        View::Overview(props) => match kind {
                            MouseEventKind::ScrollDown => {
                                props.sections.scroll_down();
                                true
                            }
                            MouseEventKind::ScrollUp => {
                                props.sections.scroll_up();
                                true
                            }
                            _ => false,
                        },
                        _ => false,
                    });

                    if !modified {
                        return Cmd::None;
                    }

                    Cmd::task(|handles| async move {
                        let result = handles.tui.render().await;
                        Msg::Tui(TuiMsg::RenderDone(result))
                    })
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
