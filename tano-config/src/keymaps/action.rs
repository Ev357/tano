use serde::Deserialize;

use crate::keymaps::direction::Direction;

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Quit,
    #[serde(untagged)]
    Move(Direction),
}
