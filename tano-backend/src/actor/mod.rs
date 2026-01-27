use std::io::{self, stdout};

use color_eyre::eyre::{Context, Result};
use crossterm::{
    event::{Event, EventStream},
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use tokio::sync::{
    mpsc,
    watch::{self},
};
use tokio_stream::StreamExt;

use crate::{
    actor::{cmd::BackendCmd, msg::BackendMsg},
    model::BackendModel,
};

pub mod cmd;
pub mod handle;
pub mod msg;

pub struct BackendActor<T: BackendModel> {
    receiver: mpsc::Receiver<BackendCmd>,
    model_rx: watch::Receiver<T>,
    msg_tx: mpsc::Sender<BackendMsg>,
    reader: EventStream,
}

impl<T: BackendModel> BackendActor<T> {
    pub fn new(
        receiver: mpsc::Receiver<BackendCmd>,
        model_rx: watch::Receiver<T>,
        msg_tx: mpsc::Sender<BackendMsg>,
    ) -> Self {
        Self {
            receiver,
            model_rx,
            msg_tx,
            reader: EventStream::new(),
        }
    }

    async fn handle_command(&mut self, cmd: BackendCmd) {
        match cmd {
            BackendCmd::Restore { respond_to } => {
                let _ = respond_to.send(self.restore());
            }
        }
    }

    fn handle_update(&self) {
        // TODO: handle update
    }

    async fn handle_event(&self, event: Result<Event, io::Error>) {
        let _ = self.msg_tx.send(BackendMsg::Event(event)).await;
    }

    fn restore(&self) -> Result<()> {
        let result: Result<()> = (|| {
            disable_raw_mode()?;
            execute!(stdout(), LeaveAlternateScreen)?;

            Ok(())
        })();

        result.context("Failed to restore terminal")
    }
}

pub async fn run_backend_actor<T: BackendModel>(mut actor: BackendActor<T>) -> Result<()> {
    loop {
        tokio::select! {
            Some(cmd) = actor.receiver.recv() => {
                actor.handle_command(cmd).await;
            }
            Ok(_) = actor.model_rx.changed() => {
                actor.handle_update();
            }
            maybe_event = actor.reader.next() => {
                match maybe_event {
                    Some(event) => {
                        actor.handle_event(event).await;
                    }
                    None => break,
                }
            }
            else => break,
        }
    }

    Ok(())
}
