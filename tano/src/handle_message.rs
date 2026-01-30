use crossterm::event::{Event, KeyCode};
use tano_backend::actor::{handle::BackendActorHandle, msg::BackendMsg};
use tano_config::actor::{handle::ConfigActorHandle, msg::ConfigMsg};
use tano_database::actor::{handle::DatabaseActorHandle, mgs::DatabaseMsg};
use tano_shared::{get_config_dir::get_config_dir, get_config_file::get_config_file};
use tano_tui::{
    actor::{handle::TuiActorHandle, msg::TuiMsg},
    components::songs::SongsProps,
    view::View,
};
use tano_watcher::{
    actor::{handle::WatcherActorHandle, msg::WatcherMsg},
    watch_type::WatchType,
};
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::{Model, config_state::ConfigState, database_state::DatabaseState},
    msg::Msg,
};

#[derive(Clone)]
pub struct Handles {
    pub tui: TuiActorHandle,
    pub config: ConfigActorHandle,
    pub database: DatabaseActorHandle,
    pub backend: BackendActorHandle,
    pub watcher: WatcherActorHandle,
}

pub fn handle_message(model_tx: &watch::Sender<Model>, msg: Msg) -> Cmd {
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
        Msg::Database(database_msg) => match database_msg {
            DatabaseMsg::SongsLoaded { songs } => match songs {
                Ok(songs) => {
                    model_tx.send_modify(|model| model.view = View::Songs(SongsProps { songs }));

                    Cmd::None
                }
                Err(report) => Cmd::Error(report),
            },
        },
        Msg::Backend(backend_msg) => match backend_msg {
            BackendMsg::Event(event) => match event {
                Ok(event) => match event {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('q') => Cmd::Msg(Msg::Restore),
                        _ => Cmd::None,
                    },
                    Event::Resize(_, _) => Cmd::task(|handles| async move {
                        let result = handles.tui.render().await;
                        Msg::Tui(TuiMsg::RenderDone(result))
                    }),
                    _ => Cmd::None,
                },
                Err(error) => Cmd::Error(error.into()),
            },
        },
        Msg::Watcher(watcher_msg) => match watcher_msg {
            WatcherMsg::FileChange {
                watch_type,
                path: _,
            } => match watch_type {
                WatchType::Config => Cmd::task(|handles| async move {
                    let result = handles.config.load_config().await;

                    Msg::Config(ConfigMsg::ConfigLoaded(result))
                }),
            },
        },
        Msg::Tui(TuiMsg::RenderDone(result)) => match result {
            Ok(()) => Cmd::None,
            Err(report) => Cmd::Error(report),
        },
        Msg::Config(config_msg) => match config_msg {
            ConfigMsg::ConfigLoaded(config) => match config {
                Ok(config) => {
                    model_tx.send_modify(|model| model.config = ConfigState::Loaded(config));

                    Cmd::None
                }
                Err(report) => Cmd::Error(report),
            },
        },
        Msg::Restore => Cmd::task(|handles| async move {
            let restore_result = handles.backend.restore().await;

            Msg::Close { restore_result }
        }),
        Msg::Close { restore_result } => match restore_result {
            Ok(()) => Cmd::Close,
            Err(report) => Cmd::Error(report),
        },
    }
}
