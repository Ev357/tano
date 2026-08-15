use tano_config::pages::page::Page;
use tano_tui::{
    components::{
        albums::AlbumsProps, artists::ArtistsProps, overview::OverviewProps, songs::SongsProps,
    },
    utils::{list_state::ListState, load_state::LoadState},
    view::View,
};
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::{Model, config_state::ConfigState},
    msg::Msg,
    update::{database::DatabaseMsg, tui::TuiMsg},
};

pub fn update_navigate(model_tx: &watch::Sender<Model>, page: Page) -> Cmd {
    let cmd = match page {
        Page::Overview => {
            let sections = match &model_tx.borrow().config {
                ConfigState::Loaded { pages, .. } => pages.overview.sections.clone(),
                _ => return Cmd::None,
            };

            let sections = ListState::new(sections, 0);

            model_tx.send_modify(|model| {
                model.view = View::Overview(OverviewProps { sections });
            });

            Cmd::None
        }
        Page::Songs => {
            model_tx.send_modify(|model| {
                model.view = View::Songs(SongsProps {
                    songs: LoadState::NotLoaded,
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
                    albums: LoadState::NotLoaded,
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
                    artists: LoadState::NotLoaded,
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
