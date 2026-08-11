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

use std::collections::HashSet;

use tano_shared::{get_config_dir::get_config_dir, get_config_file::get_config_file};
use tano_watcher::{path_type::PathType, watch_entry::WatchEntry, watch_id::WatchId};

impl WatcherModel for Model {
    fn entries(&self) -> HashSet<WatchEntry> {
        let mut entries = HashSet::new();

        if let Ok(config_dir) = get_config_dir() {
            let config_path = get_config_file(&config_dir);
            entries.insert(WatchEntry {
                id: WatchId::Config,
                path: config_path,
                path_type: PathType::File,
            });
        }

        for (index, provider) in self.providers.iter().enumerate() {
            match provider {
                ProviderType::Local(local_provider) => {
                    entries.insert(WatchEntry {
                        id: WatchId::Provider(index as u64),
                        path: local_provider.config.path.clone(),
                        path_type: PathType::Directory,
                    });
                }
            }
        }

        entries
    }
}
