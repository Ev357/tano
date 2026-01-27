use color_eyre::eyre::Result;
use tokio::sync::oneshot;

pub enum BackendCmd {
    Restore {
        respond_to: oneshot::Sender<Result<()>>,
    },
}
