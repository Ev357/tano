use tano_config::pages::page::Page;
use tano_tui::{
    components::{
        album::AlbumProps, albums::AlbumsProps, artists::ArtistsProps, overview::OverviewProps,
        songs::SongsProps,
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
    model_tx.send_if_modified(|model| {
        if let Some((page, index)) = get_view_cursor(&model.view) {
            model.last_cursor.insert(page, index);
            return true;
        }

        false
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
        Page::Album(album_id) => {
            model_tx.send_modify(|model| {
                model.view = View::Album(AlbumProps {
                    album_id,
                    data: LoadState::Loading,
                });
            });

            Cmd::task(move |handles| async move {
                let (songs, album, artists) = tokio::join!(
                    handles.database.get_album_songs(album_id),
                    handles.database.get_album(album_id),
                    handles.database.get_album_artists(album_id)
                );
                Msg::Database(DatabaseMsg::AlbumSongsLoaded {
                    album_id,
                    album,
                    artists,
                    songs,
                })
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

fn get_view_cursor(view: &View) -> Option<(Page, usize)> {
    match view {
        View::Overview(props) => props
            .sections
            .selected_index
            .map(|index| (Page::Overview, index)),
        View::Songs(props) => get_load_state_cursor(&props.songs, Page::Songs),
        View::Albums(props) => get_load_state_cursor(&props.albums, Page::Albums),
        View::Artists(props) => get_load_state_cursor(&props.artists, Page::Artists),
        View::Album(props) => {
            if let LoadState::Loaded((_, _, list)) = &props.data {
                return list
                    .selected_index
                    .map(|index| (Page::Album(props.album_id), index));
            }

            None
        }
        _ => None,
    }
}

fn get_load_state_cursor<T>(state: &LoadState<ListState<T>>, page: Page) -> Option<(Page, usize)> {
    if let LoadState::Loaded(list) = state {
        return list.selected_index.map(|index| (page, index));
    }

    None
}
