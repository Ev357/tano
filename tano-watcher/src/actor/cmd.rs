use std::path::PathBuf;

use color_eyre::eyre::Result;
use tokio::sync::oneshot;

use crate::watch_type::WatchType;

pub enum WatcherCmd {
    Watch {
        path: PathBuf,
        watch_type: WatchType,
        respond_to: oneshot::Sender<Result<()>>,
    },
    Unwatch {
        path: PathBuf,
        respond_to: oneshot::Sender<Result<()>>,
    },
}
