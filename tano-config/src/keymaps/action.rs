use serde::Deserialize;

use crate::{
    keymaps::{direction::Direction, edge::Edge},
    pages::page::Page,
    utils::deserialize_percentage::deserialize_percentage,
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
    Scroll {
        #[serde(deserialize_with = "deserialize_percentage")]
        scroll: i32,
    },
    #[serde(untagged)]
    Move(Direction),
}
