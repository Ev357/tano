use tano_providers::ProviderType;
use tano_tui::view::View;
use tokio::sync::watch::Sender;

use crate::{
    cmd::Cmd,
    model::Model,
    msg::Msg,
    update::{
        database::DatabaseMsg,
        providers::{full_sync::full_sync, msg::ProvidersMsg, sync::sync},
    },
};

mod full_sync;
pub mod msg;
mod sync;

pub fn update_providers(model_tx: &Sender<Model>, providers_msg: ProvidersMsg) -> Cmd {
    match providers_msg {
        ProvidersMsg::FullSync => {
            let local_providers: Vec<_> = model_tx
                .borrow()
                .providers
                .iter()
                .enumerate()
                .map(|(index, provider)| {
                    let path = match provider {
                        ProviderType::Local(provider) => provider.config.path.clone(),
                    };

                    (index as u64, path)
                })
                .collect();

            Cmd::task(move |handles| async move {
                let mut overall_result = Ok(());

                for (provider_id, path) in local_providers {
                    if let Err(error) = full_sync(handles.clone(), provider_id, path).await {
                        overall_result = Err(error);
                        break;
                    }
                }
                Msg::Providers(ProvidersMsg::FullSyncDone {
                    result: overall_result,
                })
            })
        }
        ProvidersMsg::FullSyncDone { result } | ProvidersMsg::SyncDone { result } => {
            if let Err(error) = result {
                return Cmd::Error(error);
            }

            match &model_tx.borrow().view {
                View::Songs(_) => Cmd::task(|handles| async move {
                    let songs = handles.database.get_songs().await;
                    Msg::Database(DatabaseMsg::SongsLoaded { songs })
                }),
                View::Albums(_) => Cmd::task(|handles| async move {
                    let albums = handles.database.get_albums().await;
                    Msg::Database(DatabaseMsg::AlbumsLoaded { albums })
                }),
                View::Artists(_) => Cmd::task(|handles| async move {
                    let artists = handles.database.get_artists().await;
                    Msg::Database(DatabaseMsg::ArtistsLoaded { artists })
                }),
                View::Album(props) => {
                    let album_id = props.album_id;

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
                View::Song(props) => {
                    let song_id = props.song_id;

                    Cmd::task(move |handles| async move {
                        let song = handles.database.get_song(song_id).await;

                        let album = match &song {
                            Ok(Some(s)) => handles.database.get_album(s.album_id).await,
                            _ => Ok(None),
                        };

                        let artists = handles.database.get_song_artists(song_id).await;

                        Msg::Database(DatabaseMsg::SongLoaded {
                            song_id,
                            song,
                            album,
                            artists,
                        })
                    })
                }
                View::Loading | View::Overview(_) => Cmd::None,
            }
        }
        ProvidersMsg::Sync { provider_id, path } => Cmd::task(move |handles| async move {
            let result = sync(handles, provider_id, path).await;
            Msg::Providers(ProvidersMsg::SyncDone { result })
        }),
    }
}
