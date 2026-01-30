use tano_database::actor::mgs::DatabaseMsg;
use tano_shared::{get_config_dir::get_config_dir, get_config_file::get_config_file};
use tano_watcher::watch_type::WatchType;
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::{Model, config_state::ConfigState, database_state::DatabaseState},
    msg::Msg,
    update::{
        backend::update_backend, config::update_config, database::update_database,
        handles::Handles, tui::update_tui, watcher::update_watcher,
    },
};

mod backend;
mod config;
mod database;
pub mod handles;
mod tui;
mod watcher;

pub fn update(model_tx: &watch::Sender<Model>, msg: Msg) -> Cmd {
    match msg {
        Msg::Init => Cmd::task(
            |Handles {
                 config,
                 database,
                 tui,
                 watcher,
                 ..
             }| async move {
                let result = tokio::try_join!(
                    config.load_config(),
                    async {
                        let config_dir = get_config_dir()?;
                        let config_path = get_config_file(&config_dir);
                        watcher.watch(config_path, WatchType::Config).await
                    },
                    database.load_database(),
                    tui.render()
                )
                .map(|(config, _, _, _)| config);

                Msg::InitDone { result }
            },
        ),
        Msg::InitDone { result } => match result {
            Ok(config) => {
                model_tx.send_modify(|model| {
                    model.config = ConfigState::Loaded(config);
                    model.database = DatabaseState::Loaded;
                });

                Cmd::task(|handles| async move {
                    let songs = handles.database.get_songs().await;

                    Msg::Database(DatabaseMsg::SongsLoaded { songs })
                })
            }
            Err(report) => Cmd::Error(report),
        },
        Msg::Restore => Cmd::task(|handles| async move {
            let restore_result = handles.backend.restore().await;

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
    }
}
