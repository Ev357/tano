use std::io::Stdout;

use color_eyre::eyre::Result;
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::{
    mpsc,
    watch::{self},
};

use crate::{
    actor::cmd::TuiCmd,
    components::root::RootComponent,
    model::TuiModel,
    utils::{restore::restore, try_init::try_init},
};

pub mod cmd;
pub mod handle;
pub mod msg;

pub struct TuiActor<T: TuiModel> {
    receiver: mpsc::Receiver<TuiCmd>,
    model_rx: watch::Receiver<T>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl<T: TuiModel> TuiActor<T> {
    pub fn new(receiver: mpsc::Receiver<TuiCmd>, model_rx: watch::Receiver<T>) -> Result<Self> {
        let terminal = try_init()?;

        Ok(Self {
            receiver,
            model_rx,
            terminal,
        })
    }

    async fn handle_command(&mut self, cmd: TuiCmd) {
        match cmd {
            TuiCmd::Render { respond_to } => {
                let _ = respond_to.send(self.render());
            }
            TuiCmd::Restore { respond_to } => {
                let _ = respond_to.send(restore());
            }
        }
    }

    fn render(&mut self) -> Result<()> {
        let model = self.model_rx.borrow();
        self.terminal
            .draw(|frame| RootComponent::render(frame, model.view()))?;

        Ok(())
    }
}

pub async fn run_tui_actor<T: TuiModel>(mut actor: TuiActor<T>) -> Result<()> {
    loop {
        tokio::select! {
            Some(cmd) = actor.receiver.recv() => {
                actor.handle_command(cmd).await;
            }
            Ok(_) = actor.model_rx.changed() => {
                actor.render()?;
            }
            else => break,
        }
    }

    Ok(())
}
