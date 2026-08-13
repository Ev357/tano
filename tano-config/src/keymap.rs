use serde::Deserialize;

use crate::{action::Action, keybind::KeyBind, one_or_many::OneOrMany};

#[derive(Debug, Clone, Deserialize)]
pub struct Keymap {
    pub on: OneOrMany<KeyBind>,
    pub run: Action,
}
