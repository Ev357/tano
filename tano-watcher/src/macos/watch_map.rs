use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use crate::watch_id::WatchId;

#[derive(Debug, Default)]
pub struct WatchMap {
    paths_map: HashMap<PathBuf, HashSet<WatchId>>,
    id_map: HashMap<WatchId, PathBuf>,
}

impl WatchMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, path: impl AsRef<Path>, id: WatchId) {
        let path_buf = path.as_ref().to_path_buf();
        self.id_map.insert(id, path_buf.clone());
        self.paths_map.entry(path_buf).or_default().insert(id);
    }

    pub fn remove_by_id(&mut self, id: WatchId) {
        let path = match self.id_map.remove(&id) {
            Some(path) => path,
            None => return,
        };

        let entries = match self.paths_map.get_mut(&path) {
            Some(entries) => entries,
            None => return,
        };

        entries.remove(&id);

        if entries.is_empty() {
            self.paths_map.remove(&path);
        }
    }

    pub fn get_matches(&self, event_path: impl AsRef<Path>) -> Vec<WatchId> {
        let mut matches = Vec::new();

        for (depth, ancestor) in event_path.as_ref().ancestors().enumerate() {
            if depth > 1 {
                break;
            }

            if let Some(entries) = self.paths_map.get(ancestor) {
                matches.extend(entries.iter().copied());
            }
        }

        matches
    }
}
