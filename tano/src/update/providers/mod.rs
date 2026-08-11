use tano_database::actor::mgs::DatabaseMsg;
use tano_providers::ProviderType;
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::Model,
    msg::Msg,
    update::providers::{full_sync::full_sync, msg::ProvidersMsg, sync::sync},
};

mod full_sync;
pub mod msg;
mod sync;

pub fn update_providers(model_tx: &watch::Sender<Model>, providers_msg: ProvidersMsg) -> Cmd {
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
        ProvidersMsg::FullSyncDone { result } | ProvidersMsg::SyncDone { result } => match result {
            Ok(_) => Cmd::task(|handles| async move {
                let songs = handles.database.get_songs().await;
                Msg::Database(DatabaseMsg::SongsLoaded { songs })
            }),
            Err(error) => Cmd::Error(error),
        },
        ProvidersMsg::Sync { provider_id, path } => Cmd::task(move |handles| async move {
            let result = sync(handles, provider_id, path).await;
            Msg::Providers(ProvidersMsg::SyncDone { result })
        }),
    }
}
