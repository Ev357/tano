use std::fmt::{self, Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Page {
    Overview,
    Songs,
    Albums,
    Album(i64),
    Song(i64),
    Artists,
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let page_str = match self {
            Page::Overview => "Overview",
            Page::Songs => "Songs",
            Page::Albums => "Albums",
            Page::Artists => "Artists",
            Page::Album(id) => &format!("Album {}", id),
            Page::Song(id) => &format!("Song {}", id),
        };
        write!(f, "{}", page_str)
    }
}
