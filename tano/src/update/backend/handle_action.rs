use tano_config::{
    keymaps::{action::Action, direction::Direction},
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
        Action::Move(direction) => {
            match &model_tx.borrow().view {
                View::Songs(_) | View::Albums(_) | View::Artists(_) => {
                    if let Direction::Left = direction {
                        return Cmd::Msg(Msg::Navigate(Page::Overview));
                    }
                }
                View::Overview(props) => {
                    if let (Direction::Right, Some(section)) =
                        (direction, props.sections.selected())
                    {
                        return Cmd::Msg(Msg::Navigate(section.clone()));
                    }
                }
                _ => {}
            }

            let modified = model_tx.send_if_modified(|model| match &mut model.view {
                View::Songs(props) => handle_load_state_navigation(&mut props.songs, direction),
                View::Albums(props) => handle_load_state_navigation(&mut props.albums, direction),
                View::Artists(props) => handle_load_state_navigation(&mut props.artists, direction),
                View::Overview(props) => {
                    handle_list_state_navigation(&mut props.sections, direction)
                }
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

fn handle_list_state_navigation<T>(list: &mut ListState<T>, direction: &Direction) -> bool {
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

fn handle_load_state_navigation<T>(
    load_state: &mut LoadState<ListState<T>>,
    direction: &Direction,
) -> bool {
    match load_state {
        LoadState::Loaded(list) => handle_list_state_navigation(list, direction),
        LoadState::Loading => false,
    }
}
