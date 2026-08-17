use tano_backend::actor::msg::BackendMsg;
use tano_config::{
    keymaps::{action::Action, direction::Direction, edge::Edge},
    pages::page::Page,
};
use tano_tui::{
    utils::{list_state::ListState, load_state::LoadState},
    view::View,
};
use tokio::sync::watch::Sender;

use crate::{cmd::Cmd, model::Model, msg::Msg, update::tui::TuiMsg};

pub fn handle_action(model_tx: &Sender<Model>, action: &Action) -> Cmd {
    match action {
        Action::Quit => Cmd::Msg(Msg::Restore),
        Action::Suspend => Cmd::Msg(Msg::Backend(BackendMsg::Suspend)),
        Action::GoTo { goto } => Cmd::Msg(Msg::Navigate(*goto)),
        Action::Jump(edge) => {
            let modified = model_tx.send_if_modified(|model| match &mut model.view {
                View::Songs(props) => handle_load_state_jump(&mut props.songs, edge),
                View::Album(props) => {
                    let list = match &mut props.data {
                        LoadState::Loaded((_, _, list)) => list,
                        LoadState::Loading => return false,
                    };
                    match edge {
                        Edge::Top => {
                            list.jump_top();
                            true
                        }
                        Edge::Bottom => {
                            list.jump_bottom();
                            true
                        }
                    }
                }
                View::Albums(props) => handle_load_state_jump(&mut props.albums, edge),
                View::Artists(props) => handle_load_state_jump(&mut props.artists, edge),
                View::Overview(props) => match edge {
                    Edge::Top => {
                        props.sections.jump_top();
                        true
                    }
                    Edge::Bottom => {
                        props.sections.jump_bottom();
                        true
                    }
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
        Action::Scroll { scroll } => {
            let modified = model_tx.send_if_modified(|model| match &mut model.view {
                View::Songs(props) => handle_load_state_scroll(&mut props.songs, *scroll),
                View::Album(props) => {
                    let list = match &mut props.data {
                        LoadState::Loaded((_, _, list)) => list,
                        LoadState::Loading => return false,
                    };
                    list.scroll_percent(*scroll);
                    true
                }
                View::Albums(props) => handle_load_state_scroll(&mut props.albums, *scroll),
                View::Artists(props) => handle_load_state_scroll(&mut props.artists, *scroll),
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
        Action::Move(direction) => {
            match &model_tx.borrow().view {
                View::Album(_) => {
                    if let Direction::Left = direction {
                        return Cmd::Msg(Msg::Navigate(Page::Albums));
                    }
                }
                View::Song(_) => {
                    if let Direction::Left = direction {
                        return Cmd::Msg(Msg::Navigate(Page::Songs));
                    }
                }
                View::Songs(props) => {
                    if let Direction::Left = direction {
                        return Cmd::Msg(Msg::Navigate(Page::Overview));
                    }
                    if let (Direction::Right, Some(song)) = (
                        direction,
                        match &props.songs {
                            LoadState::Loaded(list) => list.selected(),
                            _ => None,
                        },
                    ) {
                        return Cmd::Msg(Msg::Navigate(Page::Song(song.id)));
                    }
                }
                View::Artists(_) => {
                    if let Direction::Left = direction {
                        return Cmd::Msg(Msg::Navigate(Page::Overview));
                    }
                }
                View::Overview(props) => {
                    if let (Direction::Right, Some(section)) =
                        (direction, props.sections.selected())
                    {
                        return Cmd::Msg(Msg::Navigate(*section));
                    }
                }
                View::Albums(props) => {
                    if let Direction::Left = direction {
                        return Cmd::Msg(Msg::Navigate(Page::Overview));
                    }
                    if let (Direction::Right, Some(album)) = (
                        direction,
                        match &props.albums {
                            LoadState::Loaded(list) => list.selected(),
                            _ => None,
                        },
                    ) {
                        return Cmd::Msg(Msg::Navigate(Page::Album(album.id)));
                    }
                }
                _ => {}
            }

            let modified = model_tx.send_if_modified(|model| match &mut model.view {
                View::Songs(props) => handle_load_state_navigation(&mut props.songs, direction),
                View::Album(props) => {
                    let list = match &mut props.data {
                        LoadState::Loaded((_, _, list)) => list,
                        LoadState::Loading => return false,
                    };
                    match direction {
                        Direction::Up => {
                            list.previous();
                            true
                        }
                        Direction::Down => {
                            list.next();
                            true
                        }
                        _ => false,
                    }
                }
                View::Albums(props) => handle_load_state_navigation(&mut props.albums, direction),
                View::Artists(props) => handle_load_state_navigation(&mut props.artists, direction),
                View::Overview(props) => match direction {
                    Direction::Up => {
                        props.sections.previous();
                        true
                    }
                    Direction::Down => {
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

fn handle_load_state_navigation<T>(
    load_state: &mut LoadState<ListState<T>>,
    direction: &Direction,
) -> bool {
    let list = match load_state {
        LoadState::Loaded(list) => list,
        LoadState::Loading => return false,
    };

    match direction {
        Direction::Up => {
            list.previous();
            true
        }
        Direction::Down => {
            list.next();
            true
        }
        _ => false,
    }
}

fn handle_load_state_jump<T>(load_state: &mut LoadState<ListState<T>>, edge: &Edge) -> bool {
    let list = match load_state {
        LoadState::Loaded(list) => list,
        LoadState::Loading => return false,
    };

    match edge {
        Edge::Top => {
            list.jump_top();
        }
        Edge::Bottom => {
            list.jump_bottom();
        }
    }

    true
}

fn handle_load_state_scroll<T>(load_state: &mut LoadState<ListState<T>>, percent: i32) -> bool {
    let list = match load_state {
        LoadState::Loaded(list) => list,
        LoadState::Loading => return false,
    };

    list.scroll_percent(percent);
    true
}

pub fn handle_load_state_scroll_down<T>(load_state: &mut LoadState<ListState<T>>) -> bool {
    let list = match load_state {
        LoadState::Loaded(list) => list,
        LoadState::Loading => return false,
    };

    list.scroll_down();
    true
}

pub fn handle_load_state_scroll_up<T>(load_state: &mut LoadState<ListState<T>>) -> bool {
    let list = match load_state {
        LoadState::Loaded(list) => list,
        LoadState::Loading => return false,
    };

    list.scroll_up();
    true
}
