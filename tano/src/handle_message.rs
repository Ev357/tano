use crossterm::event::{Event, KeyCode};
use tano_backend::actor::{handle::BackendActorHandle, msg::BackendMsg};
use tano_config::actor::handle::ConfigActorHandle;
use tano_database::actor::{handle::DatabaseActorHandle, mgs::DatabaseMsg};
use tano_tui::{actor::handle::TuiActorHandle, components::songs::SongsProps, view::View};
use tokio::sync::watch;

use crate::{
    cmd::Cmd,
    model::{Model, config_state::ConfigState, database_state::DatabaseState},
    msg::Msg,
};

#[derive(Clone)]
pub struct Handles {
    pub tui_handle: TuiActorHandle,
    pub config_handle: ConfigActorHandle,
    pub database_handle: DatabaseActorHandle,
    pub backend_handle: BackendActorHandle,
}

pub fn handle_message(model_tx: &watch::Sender<Model>, msg: Msg) -> Cmd {
    match msg {
        Msg::Init => Cmd::Some(
            |Handles {
                 config_handle,
                 database_handle,
                 tui_handle,
                 ..
             }| {
                Box::pin(async move {
                    let result = tokio::try_join!(
                        config_handle.load_config(),
                        database_handle.load_database(),
                        tui_handle.render()
                    )
                    .map(|(config, _, _)| config);

                    Msg::InitDone { result }
                })
            },
        ),
        Msg::InitDone { result } => match result {
            Ok(config) => {
                model_tx.send_modify(|model| {
                    model.config = ConfigState::Loaded(config);
                    model.database = DatabaseState::Loaded;
                });

                Cmd::Some(
                    |Handles {
                         database_handle, ..
                     }| {
                        Box::pin(async move {
                            let songs = database_handle.get_songs().await;

                            Msg::Database(DatabaseMsg::SongsLoaded { songs })
                        })
                    },
                )
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
                    _ => Cmd::None,
                },
                Err(error) => Cmd::Error(error.into()),
            },
        },
        Msg::Restore => Cmd::Some(|Handles { backend_handle, .. }| {
            Box::pin(async move {
                let restore_result = backend_handle.restore().await;

                Msg::Close { restore_result }
            })
        }),
        Msg::Close { restore_result } => match restore_result {
            Ok(()) => Cmd::Close,
            Err(report) => Cmd::Error(report),
        },
    }
}
