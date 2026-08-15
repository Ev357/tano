use color_eyre::eyre::Result;
use tano_config::config::Config;
use tokio::sync::watch::Sender;

#[derive(Debug)]
pub enum ConfigMsg {
    ConfigLoaded(Result<Config>),
}

use crate::{
    cmd::Cmd,
    model::{
        Model,
        config_state::{ConfigState, ConfigWatchState},
        key_trie::KeyTrie,
    },
    msg::Msg,
};

pub fn update_config(model_tx: &Sender<Model>, config_msg: ConfigMsg) -> Cmd {
    match config_msg {
        ConfigMsg::ConfigLoaded(config) => match config {
            Ok(config) => {
                let watch_state = ConfigWatchState::resolve();

                let mut keymap_trie = KeyTrie::new();

                if let Some(keymaps) = config.keymaps {
                    for keymap in keymaps {
                        keymap_trie.insert(&keymap.on, keymap.run);
                    }
                }

                model_tx.send_modify(|model| {
                    model.config = ConfigState::Loaded {
                        pages: config.pages.clone(),
                        watch_state,
                    };
                    model.keymap = keymap_trie;
                    model.keybind_buffer.clear();
                });

                Cmd::Batch(vec![
                    Cmd::Msg(Msg::InitProviders {
                        config: config.providers,
                    }),
                    Cmd::Msg(Msg::InitInitialView {
                        startup_page: config.startup_page,
                    }),
                    Cmd::Msg(Msg::RefreshView),
                ])
            }
            Err(report) => Cmd::Error(report),
        },
    }
}
