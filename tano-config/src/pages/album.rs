use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AlbumPage {
    pub hide_track_numbers: bool,
    pub hide_artists: bool,
}
