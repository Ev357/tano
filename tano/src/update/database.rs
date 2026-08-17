use color_eyre::eyre::Result;
use tano_config::pages::page::Page;
use tano_database::{album::Album, artist::Artist, song::Song};
use tano_tui::{
    components::{
        album::AlbumProps, albums::AlbumsProps, artists::ArtistsProps, song::SongProps,
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
};

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum DatabaseMsg {
    SongsLoaded {
        songs: Result<Vec<Song>>,
    },
    AlbumsLoaded {
        albums: Result<Vec<Album>>,
    },
    AlbumSongsLoaded {
        album_id: i64,
        album: Result<Option<Album>>,
        artists: Result<Vec<Artist>>,
        songs: Result<Vec<Song>>,
    },
    ArtistsLoaded {
        artists: Result<Vec<Artist>>,
    },
    SongLoaded {
        song_id: i64,
        song: Result<Option<Song>>,
        album: Result<Option<Album>>,
        artists: Result<Vec<Artist>>,
    },
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
        DatabaseMsg::AlbumSongsLoaded {
            album_id,
            album,
            artists,
            songs,
        } => match (album, artists, songs) {
            (Ok(Some(album)), Ok(artists), Ok(songs)) => {
                model_tx.send_if_modified(|model| {
                    let cursor = model
                        .last_cursor
                        .get(&Page::Album(album_id))
                        .copied()
                        .unwrap_or(0);
                    let songs = ListState::new(songs, cursor);

                    let config = match &model.config {
                        ConfigState::Loaded { pages, .. } => pages.album.clone(),
                        _ => return false,
                    };

                    model.view = View::Album(AlbumProps {
                        album_id,
                        config,
                        data: LoadState::Loaded((album, artists, songs)),
                    });
                    true
                });

                Cmd::None
            }
            (Ok(None), _, _) => Cmd::Msg(Msg::Navigate(Page::Albums)),
            (Err(report), _, _) | (_, Err(report), _) | (_, _, Err(report)) => Cmd::Error(report),
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
        DatabaseMsg::SongLoaded {
            song_id,
            song,
            album,
            artists,
        } => match (song, album, artists) {
            (Ok(Some(song)), Ok(album), Ok(artists)) => {
                model_tx.send_if_modified(|model| {
                    if let Some((Page::Song(current_id), _)) =
                        crate::update::navigate::get_view_cursor(&model.view)
                    {
                        if current_id != song_id {
                            return false;
                        }
                    }

                    model.view = View::Song(SongProps {
                        song_id,
                        data: LoadState::Loaded((song, album, artists)),
                    });
                    true
                });

                Cmd::None
            }
            (Ok(None), _, _) => Cmd::Msg(Msg::Navigate(Page::Songs)),
            (Err(report), _, _) | (_, Err(report), _) | (_, _, Err(report)) => Cmd::Error(report),
        },
    }
}
