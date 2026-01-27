use color_eyre::eyre::Result;
use tokio::sync::oneshot;

pub enum TuiCmd {
    Render {
        respond_to: oneshot::Sender<Result<()>>,
    },
}
