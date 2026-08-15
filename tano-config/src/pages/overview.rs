use serde::Deserialize;

use crate::pages::page::Page;

#[derive(Debug, Deserialize, Clone)]
pub struct OverviewPage {
    pub sections: Vec<Page>,
}
