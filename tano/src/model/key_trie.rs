use std::collections::HashMap;

use tano_config::keymaps::{action::Action, keybind::KeyBind};

#[derive(Debug, Clone, Default)]
pub struct KeyTrie {
    pub action: Option<Action>,
    pub children: HashMap<KeyBind, KeyTrie>,
}

impl KeyTrie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, sequence: &[KeyBind], action: Action) {
        let mut current = self;

        for key in sequence {
            current = current.children.entry(*key).or_default();
        }

        current.action = Some(action);
    }
}
