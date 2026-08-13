use std::collections::HashSet;

use tano_backend::model::BackendModel;
use tano_config::keybind::KeyBind;
use tano_providers::ProviderType;
use tano_shared::get_config_dir::get_config_dir;
use tano_tui::{model::TuiModel, view::View};
use tano_watcher::{
    model::WatcherModel, watch_entry::WatchEntry, watch_filter::WatchFilter, watch_id::WatchId,
};

use crate::model::{
    config_state::{ConfigState, ConfigWatchState},
    database_state::DatabaseState,
    key_trie::KeyTrie,
};

pub mod config_state;
pub mod database_state;
pub mod key_trie;

#[derive(Default, Debug)]
pub struct Model {
    pub config: ConfigState,
    pub database: DatabaseState,
    pub view: View,
    pub providers: Vec<ProviderType>,
    pub keymap: KeyTrie,
    pub keybind_buffer: Vec<KeyBind>,
}

impl TuiModel for Model {
    fn view(&self) -> &View {
        &self.view
    }
}

impl BackendModel for Model {}

impl WatcherModel for Model {
    fn entries(&self) -> HashSet<WatchEntry> {
        let mut entries = HashSet::new();

        if let ConfigState::Loaded(watch_state) = &self.config
            && let Ok(config_dir) = get_config_dir()
        {
            let (path, filter) = match watch_state {
                ConfigWatchState::TargetResolved => (Some(config_dir), WatchFilter::empty()),
                ConfigWatchState::FallbackConfig => (
                    config_dir.ancestors().nth(1).map(|path| path.to_path_buf()),
                    WatchFilter::IGNORE_FILES,
                ),
                ConfigWatchState::FallbackHome => (
                    config_dir.ancestors().nth(2).map(|path| path.to_path_buf()),
                    WatchFilter::IGNORE_FILES,
                ),
                ConfigWatchState::NoHome => (None, WatchFilter::empty()),
            };

            if let Some(path) = path {
                entries.insert(WatchEntry {
                    id: WatchId::Config,
                    path,
                    filter,
                });
            }
        }

        for (index, provider) in self.providers.iter().enumerate() {
            match provider {
                ProviderType::Local(local_provider) => {
                    entries.insert(WatchEntry {
                        id: WatchId::Provider(index as u64),
                        path: local_provider.config.path.clone(),
                        filter: WatchFilter::IGNORE_DIRECTORIES,
                    });
                }
            }
        }

        entries
    }
}
