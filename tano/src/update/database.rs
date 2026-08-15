use color_eyre::eyre::Result;
use tano_config::pages::page::Page;
use tano_database::{album::Album, artist::Artist, song::Song};
use tano_tui::{
    components::{albums::AlbumsProps, artists::ArtistsProps, songs::SongsProps},
    utils::{list_state::ListState, load_state::LoadState},
    view::View,
};
use tokio::sync::watch::Sender;

use crate::{cmd::Cmd, model::Model};

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum DatabaseMsg {
    SongsLoaded { songs: Result<Vec<Song>> },
    AlbumsLoaded { albums: Result<Vec<Album>> },
    ArtistsLoaded { artists: Result<Vec<Artist>> },
}

pub fn update_database(model_tx: &Sender<Model>, database_msg: DatabaseMsg) -> Cmd {
    match database_msg {
        DatabaseMsg::SongsLoaded { songs } => match songs {
            Ok(songs) => {
                model_tx.send_modify(|model| {
                    let cursor = model.last_cursor.get(&Page::Songs).copied().unwrap_or(0);
                    let songs = ListState::new(songs, cursor);
                    model.view = View::Songs(SongsProps {
                        songs: LoadState::Loaded(songs),
                    })
                });

                Cmd::None
            }
            Err(report) => Cmd::Error(report),
        },
        DatabaseMsg::AlbumsLoaded { albums } => match albums {
            Ok(albums) => {
                model_tx.send_modify(|model| {
                    let cursor = model.last_cursor.get(&Page::Albums).copied().unwrap_or(0);
                    let albums = ListState::new(albums, cursor);
                    model.view = View::Albums(AlbumsProps {
                        albums: LoadState::Loaded(albums),
                    })
                });

                Cmd::None
            }
            Err(report) => Cmd::Error(report),
        },
        DatabaseMsg::ArtistsLoaded { artists } => match artists {
            Ok(artists) => {
                model_tx.send_modify(|model| {
                    let cursor = model.last_cursor.get(&Page::Artists).copied().unwrap_or(0);
                    let artists = ListState::new(artists, cursor);
                    model.view = View::Artists(ArtistsProps {
                        artists: LoadState::Loaded(artists),
                    })
                });

                Cmd::None
            }
            Err(report) => Cmd::Error(report),
        },
    }
}
