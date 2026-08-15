use tano_config::{keymaps::action::Action, pages::page::Page};
use tano_tui::{utils::load_state::LoadState, view::View};
use tokio::sync::watch::Sender;

use crate::{cmd::Cmd, model::Model, msg::Msg, update::tui::TuiMsg};

pub fn handle_action(model_tx: &Sender<Model>, action: &Action) -> Cmd {
    match action {
        Action::Quit => Cmd::Msg(Msg::Restore),
        action @ (Action::Up | Action::Down | Action::Left | Action::Right) => {
            match &model_tx.borrow().view {
                View::Songs(_) | View::Albums(_) => {
                    if let Action::Left = action {
                        return Cmd::Msg(Msg::Navigate(Page::Overview));
                    }
                }
                View::Overview(props) => {
                    if let (Action::Right, Some(section)) = (action, props.sections.selected()) {
                        return Cmd::Msg(Msg::Navigate(section.clone()));
                    }
                }
                _ => {}
            }

            let modified = model_tx.send_if_modified(|model| match &mut model.view {
                View::Songs(props) => {
                    let songs = match &mut props.songs {
                        LoadState::Loaded(songs) => songs,
                        _ => return false,
                    };

                    match action {
                        Action::Up => {
                            songs.previous();
                            true
                        }
                        Action::Down => {
                            songs.next();
                            true
                        }
                        _ => false,
                    }
                }
                View::Albums(props) => {
                    let albums = match &mut props.albums {
                        LoadState::Loaded(albums) => albums,
                        _ => return false,
                    };

                    match action {
                        Action::Up => {
                            albums.previous();
                            true
                        }
                        Action::Down => {
                            albums.next();
                            true
                        }
                        _ => false,
                    }
                }
                View::Overview(props) => match action {
                    Action::Up => {
                        props.sections.previous();
                        true
                    }
                    Action::Down => {
                        props.sections.next();
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
    }
}
