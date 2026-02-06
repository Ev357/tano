use tano_backend::model::BackendModel;
use tano_providers::ProviderType;
use tano_tui::{model::TuiModel, view::View};
use tano_watcher::model::WatcherModel;

use crate::model::{config_state::ConfigState, database_state::DatabaseState};

pub mod config_state;
pub mod database_state;

#[derive(Default, Debug)]
pub struct Model {
    pub config: ConfigState,
    pub database: DatabaseState,
    pub view: View,
    pub providers: Vec<ProviderType>,
}

impl TuiModel for Model {
    fn view(&self) -> &View {
        &self.view
    }
}

impl BackendModel for Model {}

impl WatcherModel for Model {
    fn providers(&self) -> &[ProviderType] {
        &self.providers
    }
}
