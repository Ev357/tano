use serde::Deserialize;

use crate::{
    keymaps::{direction::Direction, edge::Edge},
    pages::page::Page,
};

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Quit,
    #[serde(untagged)]
    Jump(Edge),
    #[serde(untagged)]
    GoTo {
        goto: Page,
    },
    #[serde(untagged)]
    Move(Direction),
}
