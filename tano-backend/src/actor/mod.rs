use std::io::{self};

use color_eyre::eyre::Result;
use crossterm::event::{Event, EventStream};
use tokio::{
    signal::unix::{Signal, SignalKind, signal},
    sync::{
        mpsc,
        watch::{self},
    },
};
use tokio_stream::StreamExt;

use crate::{
    actor::{cmd::BackendCmd, msg::BackendMsg},
    model::BackendModel,
    utils::suspend::suspend,
};

pub mod cmd;
pub mod handle;
pub mod msg;

pub struct BackendActor<T: BackendModel> {
    receiver: mpsc::Receiver<BackendCmd>,
    model_rx: watch::Receiver<T>,
    msg_tx: mpsc::Sender<BackendMsg>,
    reader: EventStream,
    sigtstp: Signal,
    sigcont: Signal,
}

impl<T: BackendModel> BackendActor<T> {
    pub fn new(
        receiver: mpsc::Receiver<BackendCmd>,
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<BackendMsg>,
    ) -> Result<Self> {
        let sigtstp = signal(SignalKind::from_raw(libc::SIGTSTP))?;
        let sigcont = signal(SignalKind::from_raw(libc::SIGCONT))?;

        Ok(Self {
            receiver,
            model_rx,
            msg_tx,
            reader: EventStream::new(),
            sigtstp,
            sigcont,
        })
    }

    async fn handle_command(&mut self, cmd: BackendCmd) {
        match cmd {
            BackendCmd::Suspend { respond_to } => {
                suspend();
                let _ = respond_to.send(());
            }
        }
    }

    fn handle_update(&self) {
        // TODO: handle update
    }

    async fn handle_event(&self, event: Result<Event, io::Error>) {
        let _ = self.msg_tx.send(BackendMsg::Event(event)).await;
    }
}

pub async fn run_backend_actor<T: BackendModel>(mut actor: BackendActor<T>) {
    loop {
        let result = tokio::select! {
            Some(cmd) = actor.receiver.recv() => {
                actor.handle_command(cmd).await;

                Ok(())
            }
            Ok(_) = actor.model_rx.changed() => {
                actor.handle_update();

                Ok(())
            }
            maybe_event = actor.reader.next() => {
                match maybe_event {
                    Some(event) => {
                        actor.handle_event(event).await;

                        Ok(())
                    }
                    None => return,
                }
            }
            _ = actor.sigtstp.recv() => {
                let _ = actor.msg_tx.send(BackendMsg::Suspend).await;
                Ok(())
            }
            _ = actor.sigcont.recv() => {
                let _ = actor.msg_tx.send(BackendMsg::Resume).await;
                Ok(())
            }
            else => return,
        };

        if let Err(error) = result {
            let _ = actor.msg_tx.send(BackendMsg::Error(error)).await;
            return;
        }
    }
}
