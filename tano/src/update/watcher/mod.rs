use tano_config::actor::msg::ConfigMsg;
use tano_database::actor::mgs::DatabaseMsg;
use tano_providers::{ProviderType, local::LocalProvider};
use tano_watcher::{
    actor::msg::WatcherMsg,
    watch_type::{WatchProvider, WatchType},
};
use tokio::sync::watch;

use crate::{cmd::Cmd, model::Model, msg::Msg, update::watcher::get_all_songs::get_all_songs};

mod get_all_songs;

pub fn update_watcher(model_tx: &watch::Sender<Model>, watcher_msg: WatcherMsg) -> Cmd {
    match watcher_msg {
        WatcherMsg::FileChange {
            watch_type,
            path: _,
        } => match watch_type {
            WatchType::Config => Cmd::task(|handles| async move {
                let result = handles.config.load_config().await;

                Msg::Config(ConfigMsg::ConfigLoaded(result))
            }),
            WatchType::Provider(watch_provider) => match watch_provider {
                WatchProvider::Local => {
                    let model = model_tx.borrow();

                    let local_providers: Vec<LocalProvider> = model
                        .providers
                        .iter()
                        .filter_map(|provider| {
                            #[allow(irrefutable_let_patterns)]
                            if let ProviderType::Local(provider) = provider {
                                return Some(provider.clone());
                            }

                            None
                        })
                        .collect();

                    Cmd::task(|handles| async move {
                        let songs = match get_all_songs(local_providers).await {
                            Ok(songs) => songs,
                            Err(report) => {
                                return Msg::Database(DatabaseMsg::SongsLoaded {
                                    songs: Err(report),
                                });
                            }
                        };

                        if let Err(report) = handles.database.sync_songs(songs).await {
                            return Msg::Database(DatabaseMsg::SongsLoaded { songs: Err(report) });
                        }

                        let songs = handles.database.get_songs().await;

                        Msg::Database(DatabaseMsg::SongsLoaded { songs })
                    })
                }
            },
        },
    }
}
