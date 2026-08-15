use serde::Deserialize;

use crate::pages::overview::OverviewPage;

pub mod overview;
pub mod page;

#[derive(Debug, Deserialize, Clone)]
pub struct Pages {
    pub overview: OverviewPage,
}
