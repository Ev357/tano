use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;
use tokio::sync::{
    mpsc,
    watch::{self},
};

use crate::{actor::cmd::TuiCmd, components::root::RootComponent, model::TuiModel};

pub mod cmd;
pub mod handle;
pub mod msg;

pub struct TuiActor<T: TuiModel> {
    receiver: mpsc::Receiver<TuiCmd>,
    model_rx: watch::Receiver<T>,
    terminal: DefaultTerminal,
}

impl<T: TuiModel> TuiActor<T> {
    pub fn new(receiver: mpsc::Receiver<TuiCmd>, model_rx: watch::Receiver<T>) -> Result<Self> {
        let terminal = ratatui::try_init()?;

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
