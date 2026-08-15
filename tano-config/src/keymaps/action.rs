use serde::Deserialize;

use crate::{keymaps::direction::Direction, pages::page::Page};

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Quit,
    #[serde(untagged)]
    GoTo {
        goto: Page,
    },
    #[serde(untagged)]
    Move(Direction),
}
