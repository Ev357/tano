use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;
use tokio::sync::{
    mpsc,
    watch::{self},
};

use crate::{
    actor::cmd::TuiCmd,
    components::{component::Component, root::RootComponent},
    model::TuiModel,
};

pub mod cmd;
pub mod handle;

pub struct TuiActor<T: TuiModel> {
    receiver: mpsc::Receiver<TuiCmd>,
    model_rx: watch::Receiver<T>,
    terminal: DefaultTerminal,
    root: RootComponent,
}

impl<T: TuiModel> TuiActor<T> {
    pub fn new(receiver: mpsc::Receiver<TuiCmd>, model_rx: watch::Receiver<T>) -> Result<Self> {
        let terminal = ratatui::try_init()?;

        Ok(Self {
            receiver,
            model_rx,
            terminal,
            root: RootComponent::default(),
        })
    }

    async fn handle_command(&mut self, cmd: TuiCmd) {
        match cmd {
            TuiCmd::Render { respond_to } => {
                let _ = respond_to.send(self.render());
            }
        }
    }

    fn handle_update(&mut self) -> Result<()> {
        let model = self.model_rx.borrow();
        self.terminal
            .draw(|frame| self.root.rerender(frame, model.view()))?;

        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        self.terminal.draw(|frame| self.root.render(frame))?;

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
                actor.handle_update()?;
            }
            else => break,
        }
    }

    Ok(())
}
