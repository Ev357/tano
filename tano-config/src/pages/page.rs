use std::fmt::{self, Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Page {
    Overview,
    Songs,
    // Albums,
    // Artists,
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let page_str = match self {
            Page::Overview => "Overview",
            Page::Songs => "Songs",
            // Page::Albums => "Albums",
            // Page::Artists => "Artists",
        };
        write!(f, "{}", page_str)
    }
}
