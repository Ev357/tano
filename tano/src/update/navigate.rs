use tano_config::pages::page::Page;
use tano_tui::{
    components::{
        albums::AlbumsProps, artists::ArtistsProps, overview::OverviewProps, songs::SongsProps,
    },
    utils::{list_state::ListState, load_state::LoadState},
    view::View,
};
use tokio::sync::watch::Sender;

use crate::{
    cmd::Cmd,
    model::{Model, config_state::ConfigState},
    msg::Msg,
    update::{database::DatabaseMsg, tui::TuiMsg},
};

pub fn update_navigate(model_tx: &Sender<Model>, page: Page) -> Cmd {
    model_tx.send_if_modified(|model| match &model.view {
        View::Overview(props) => {
            if let Some(index) = props.sections.selected_index {
                model.last_cursor.insert(Page::Overview, index);
                return true;
            }

            false
        }
        View::Songs(props) => {
            if let LoadState::Loaded(list) = &props.songs
                && let Some(index) = list.selected_index
            {
                model.last_cursor.insert(Page::Songs, index);
                return true;
            }

            false
        }
        View::Albums(props) => {
            if let LoadState::Loaded(list) = &props.albums
                && let Some(index) = list.selected_index
            {
                model.last_cursor.insert(Page::Albums, index);
                return true;
            }

            false
        }
        View::Artists(props) => {
            if let LoadState::Loaded(list) = &props.artists
                && let Some(index) = list.selected_index
            {
                model.last_cursor.insert(Page::Artists, index);
                return true;
            }

            false
        }
        _ => false,
    });

    let cmd = match page {
        Page::Overview => {
            model_tx.send_if_modified(|model| {
                let sections = match &model.config {
                    ConfigState::Loaded { pages, .. } => {
                        let cursor = model.last_cursor.get(&Page::Overview).copied().unwrap_or(0);
                        ListState::new(pages.overview.sections.clone(), cursor)
                    }
                    _ => return false,
                };
                model.view = View::Overview(OverviewProps { sections });
                true
            });

            Cmd::None
        }
        Page::Songs => {
            model_tx.send_modify(|model| {
                model.view = View::Songs(SongsProps {
                    songs: LoadState::Loading,
                });
            });

            Cmd::task(|handles| async move {
                let songs = handles.database.get_songs().await;
                Msg::Database(DatabaseMsg::SongsLoaded { songs })
            })
        }
        Page::Albums => {
            model_tx.send_modify(|model| {
                model.view = View::Albums(AlbumsProps {
                    albums: LoadState::Loading,
                });
            });

            Cmd::task(|handles| async move {
                let albums = handles.database.get_albums().await;
                Msg::Database(DatabaseMsg::AlbumsLoaded { albums })
            })
        }
        Page::Artists => {
            model_tx.send_modify(|model| {
                model.view = View::Artists(ArtistsProps {
                    artists: LoadState::Loading,
                });
            });

            Cmd::task(|handles| async move {
                let artists = handles.database.get_artists().await;
                Msg::Database(DatabaseMsg::ArtistsLoaded { artists })
            })
        }
    };

    Cmd::Batch(vec![
        Cmd::task(|handles| async move {
            let result = handles.tui.render().await;
            Msg::Tui(TuiMsg::RenderDone(result))
        }),
        cmd,
    ])
}
