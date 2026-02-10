use color_eyre::eyre::Result;
use tano_backend::actor::{handle::BackendActorHandle, msg::BackendMsg};
use tano_config::actor::handle::ConfigActorHandle;
use tano_database::actor::handle::DatabaseActorHandle;
use tano_tui::actor::handle::TuiActorHandle;
use tano_watcher::actor::{handle::WatcherActorHandle, msg::WatcherMsg};
use tokio::sync::{mpsc, watch};

use crate::{
    cmd::Cmd,
    logging::initialize_logging,
    model::Model,
    msg::Msg,
    update::{handles::Handles, update},
};

mod cmd;
mod logging;
mod model;
mod msg;
mod update;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    run().await
}

async fn run() -> Result<()> {
    let _guard = initialize_logging().await?;

    let (model_tx, model_rx) = watch::channel(Model::default());
    let (msg_tx, mut msg_rx) = mpsc::channel::<Msg>(8);

    let config_handle = ConfigActorHandle::default();
    let database_handle = DatabaseActorHandle::default();
    let tui_handle = TuiActorHandle::new(model_rx.clone())?;
    let (backend_msg_tx, mut backend_msg_rx) = mpsc::channel::<BackendMsg>(8);
    let backend_handle = BackendActorHandle::new(model_rx.clone(), backend_msg_tx);
    let (watcher_msg_tx, mut watcher_msg_rx) = mpsc::channel::<WatcherMsg>(8);

    let watcher_handle = WatcherActorHandle::new(model_rx, watcher_msg_tx)?;

    let handles = Handles {
        tui: tui_handle,
        config: config_handle,
        database: database_handle,
        backend: backend_handle,
        watcher: watcher_handle,
    };

    let _ = msg_tx.send(Msg::Init).await;

    loop {
        let command = tokio::select! {
            Some(msg) = msg_rx.recv() => {
                update(&model_tx, msg)
            }
            Some(backend_msg) = backend_msg_rx.recv() => {
                update(&model_tx, Msg::Backend(backend_msg))
            }
            Some(watcher_msg) = watcher_msg_rx.recv() => {
                update(&model_tx, Msg::Watcher(watcher_msg))
            }
            else => break
        };

        let mut command_stack = vec![command];

        while let Some(cmd) = command_stack.pop() {
            match cmd {
                Cmd::Task(action) => {
                    let handles = handles.clone();
                    let tx = msg_tx.clone();
                    tokio::spawn(async move {
                        let msg = action(handles).await;
                        let _ = tx.send(msg).await;
                    });
                }
                Cmd::Msg(msg) => {
                    let _ = msg_tx.send(msg).await;
                }
                Cmd::Batch(cmds) => cmds
                    .into_iter()
                    .rev()
                    .for_each(|cmd| command_stack.push(cmd)),
                Cmd::Close => {
                    msg_rx.close();
                    return Ok(());
                }
                Cmd::Error(report) => {
                    handles.tui.restore().await?;
                    return Err(report);
                }
                Cmd::None => {}
            }
        }
    }

    Ok(())
}
