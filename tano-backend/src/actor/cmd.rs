use tokio::sync::oneshot;

pub enum BackendCmd {
    Suspend { respond_to: oneshot::Sender<()> },
}
