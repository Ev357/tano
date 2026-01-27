use color_eyre::eyre::Result;
use tano_backend::actor::{handle::BackendActorHandle, msg::BackendMsg};
use tano_config::actor::handle::ConfigActorHandle;
use tano_database::actor::handle::DatabaseActorHandle;
use tano_tui::actor::handle::TuiActorHandle;
use tokio::sync::{mpsc, watch};

use crate::{
    cmd::Cmd,
    handle_message::{Handles, handle_message},
    model::Model,
    msg::Msg,
};

mod cmd;
mod handle_message;
mod model;
mod msg;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    run().await
}

async fn run() -> Result<()> {
    let (model_tx, model_rx) = watch::channel(Model::default());
    let (msg_tx, mut msg_rx) = mpsc::channel::<Msg>(8);

    let config_handle = ConfigActorHandle::default();
    let database_handle = DatabaseActorHandle::default();
    let tui_handle = TuiActorHandle::new(model_rx.clone())?;
    let (backend_msg_tx, mut backend_msg_rx) = mpsc::channel::<BackendMsg>(8);
    let backend_handle = BackendActorHandle::new(model_rx, backend_msg_tx);

    let handles = Handles {
        tui_handle,
        config_handle,
        database_handle,
        backend_handle,
    };

    let _ = msg_tx.send(Msg::Init).await;

    loop {
        let command = tokio::select! {
            Some(msg) = msg_rx.recv() => {
                handle_message(&model_tx, msg)
            }
            Some(backend_msg) = backend_msg_rx.recv() => {
                handle_message(&model_tx, Msg::Backend(backend_msg))
            }
            else => break,
        };

        match command {
            Cmd::Some(action) => {
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
            Cmd::Close => {
                msg_rx.close();
                break;
            }
            Cmd::Error(report) => {
                return Err(report);
            }
            Cmd::None => {}
        }
    }

    Ok(())
}
