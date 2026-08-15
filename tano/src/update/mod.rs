use tano_tui::view::View;
use tokio::sync::watch::Sender;

use crate::{
    cmd::Cmd,
    model::{Model, database_state::DatabaseState},
    msg::Msg,
    update::{
        backend::update_backend,
        config::{ConfigMsg, update_config},
        database::update_database,
        navigate::update_navigate,
        providers::{msg::ProvidersMsg, update_providers},
        refresh_view::update_refresh_view,
        tui::{TuiMsg, update_tui},
        watcher::update_watcher,
    },
};

pub mod backend;
pub mod config;
pub mod database;
pub mod handles;
pub mod navigate;
pub mod providers;
pub mod refresh_view;
pub mod tui;
pub mod watcher;

pub fn update(model_tx: &Sender<Model>, msg: Msg) -> Cmd {
    match msg {
        Msg::Init => Cmd::Batch(vec![
            Cmd::task(|handles| async move {
                let result = tokio::try_join!(
                    handles.config.load_config(),
                    handles.database.load_database()
                )
                .map(|(config, _)| config);

                Msg::CoreDataLoaded { result }
            }),
            Cmd::task(|handles| async move {
                let result = handles.tui.render().await;
                Msg::Tui(TuiMsg::RenderDone(result))
            }),
        ]),
        Msg::CoreDataLoaded { result } => match result {
            Ok(_) => {
                model_tx.send_modify(|model| {
                    model.database = DatabaseState::Loaded;
                });

                Cmd::Msg(Msg::Config(ConfigMsg::ConfigLoaded(result)))
            }
            Err(report) => Cmd::Error(report),
        },
        Msg::InitProviders { config } => {
            model_tx.send_modify(|model| {
                model.providers = config.into_iter().map(Into::into).collect();
            });

            Cmd::Msg(Msg::Providers(ProvidersMsg::FullSync))
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
        Msg::InitInitialView { startup_page } => {
            if !matches!(model_tx.borrow().view, View::Loading) {
                return Cmd::None;
            }

            Cmd::Msg(Msg::Navigate(startup_page))
        }
        Msg::Navigate(page) => update_navigate(model_tx, page),
        Msg::RefreshView => update_refresh_view(model_tx),
    }
}
