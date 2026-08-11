use tano_shared::get_config_dir::get_config_dir;

#[derive(Debug)]
pub enum ConfigWatchState {
    NoHome,
    FallbackHome,
    FallbackConfig,
    TargetResolved,
}

impl ConfigWatchState {
    pub fn resolve() -> Self {
        let Ok(config_dir) = get_config_dir() else {
            return Self::NoHome;
        };

        if config_dir.exists() {
            return Self::TargetResolved;
        }

        if let Some(parent) = config_dir.parent() {
            if parent.exists() {
                return Self::FallbackConfig;
            }
            if let Some(grandparent) = parent.parent()
                && grandparent.exists()
            {
                return Self::FallbackHome;
            }
        }

        Self::NoHome
    }
}

#[derive(Default, Debug)]
pub enum ConfigState {
    #[default]
    Loading,
    Loaded(ConfigWatchState),
}
