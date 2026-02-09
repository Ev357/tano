use tano_database::actor::mgs::DatabaseMsg;
use tano_providers::{ProviderType, local::LocalProvider};
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::Model,
    msg::Msg,
    update::providers::{get_all_songs::get_all_songs, msg::ProvidersMsg},
};

mod get_all_songs;
pub mod msg;

pub fn update_providers(model_tx: &watch::Sender<Model>, providers_msg: ProvidersMsg) -> Cmd {
    match providers_msg {
        ProvidersMsg::Sync => {
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
                        return Msg::Database(DatabaseMsg::SongsLoaded { songs: Err(report) });
                    }
                };

                if let Err(report) = handles.database.sync_songs(songs).await {
                    return Msg::Database(DatabaseMsg::SongsLoaded { songs: Err(report) });
                }

                let songs = handles.database.get_songs().await;

                Msg::Database(DatabaseMsg::SongsLoaded { songs })
            })
        }
    }
}
