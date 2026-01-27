use sqlx::prelude::FromRow;

#[derive(Debug, PartialEq, Clone, FromRow)]
pub struct Song {
    pub id: i64,
    pub title: String,
    pub provider_id: String,
    pub path: String,
}
