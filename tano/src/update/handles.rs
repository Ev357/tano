use tano_backend::actor::handle::BackendActorHandle;
use tano_config::actor::handle::ConfigActorHandle;
use tano_database::actor::handle::DatabaseActorHandle;
use tano_tui::actor::handle::TuiActorHandle;
use tano_watcher::actor::handle::WatcherActorHandle;

#[derive(Clone)]
pub struct Handles {
    pub tui: TuiActorHandle,
    pub config: ConfigActorHandle,
    pub database: DatabaseActorHandle,
    pub backend: BackendActorHandle,
    #[allow(unused)]
    pub watcher: WatcherActorHandle,
}
