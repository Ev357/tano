use sqlx::{QueryBuilder, Sqlite};

pub trait BulkBuilder {
    type Item<'a>;
    fn push(&mut self, item: Self::Item<'_>);
    fn build(self) -> QueryBuilder<Sqlite>;
}
