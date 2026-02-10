use tano_config::actor::msg::ConfigMsg;
use tano_database::actor::mgs::DatabaseMsg;
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::{Model, database_state::DatabaseState},
    msg::Msg,
    update::{
        backend::update_backend,
        config::update_config,
        database::update_database,
        handles::Handles,
        providers::{msg::ProvidersMsg, update_providers},
        tui::update_tui,
        utils::init_providers::init_providers,
        watcher::update_watcher,
    },
};

mod backend;
mod config;
mod database;
pub mod handles;
pub mod providers;
mod tui;
mod utils;
mod watcher;

pub fn update(model_tx: &watch::Sender<Model>, msg: Msg) -> Cmd {
    match msg {
        Msg::Init => Cmd::task(
            |Handles {
                 config,
                 database,
                 tui,
                 ..
             }| async move {
                let result =
                    tokio::try_join!(config.load_config(), database.load_database(), tui.render())
                        .map(|(config, _, _)| config);

                Msg::InitDone { result }
            },
        ),
        Msg::InitDone { result } => match result {
            Ok(config) => {
                model_tx.send_modify(|model| {
                    model.database = DatabaseState::Loaded;
                });

                Cmd::Batch(vec![
                    Cmd::task(|handles| async move {
                        let songs = handles.database.get_songs().await;

                        Msg::Database(DatabaseMsg::SongsLoaded { songs })
                    }),
                    Cmd::Msg(Msg::Config(ConfigMsg::ConfigLoaded(Ok(config)))),
                ])
            }
            Err(report) => Cmd::Error(report),
        },
        Msg::InitProviders { config } => {
            model_tx.send_modify(|model| {
                init_providers(model, config);
            });

            Cmd::Msg(Msg::Providers(ProvidersMsg::Sync))
        }
        Msg::Restore => Cmd::task(|handles| async move {
            let restore_result = handles.tui.restore().await;

            Msg::Close { restore_result }
        }),
        Msg::Close { restore_result } => match restore_result {
            Ok(()) => Cmd::Close,
            Err(report) => Cmd::Error(report),
        },
        Msg::Database(database_msg) => update_database(model_tx, database_msg),
        Msg::Backend(backend_msg) => update_backend(model_tx, backend_msg),
        Msg::Watcher(watcher_msg) => update_watcher(model_tx, watcher_msg),
        Msg::Tui(tui_msg) => update_tui(model_tx, tui_msg),
        Msg::Config(config_msg) => update_config(model_tx, config_msg),
        Msg::Providers(providers_msg) => update_providers(model_tx, providers_msg),
    }
}
