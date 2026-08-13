use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Quit,
    Previous,
    Next,
}
