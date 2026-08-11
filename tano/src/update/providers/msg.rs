use std::path::PathBuf;

use color_eyre::eyre::Report;

#[derive(Debug)]
pub enum ProvidersMsg {
    FullSync,
    FullSyncDone { result: Result<(), Report> },
    Sync { provider_id: u64, path: PathBuf },
    SyncDone { result: Result<(), Report> },
}
