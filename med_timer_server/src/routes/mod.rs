use sqlx::{query::QueryAs, Sqlite};

pub(crate) mod entries;
pub(crate) mod meds;

/// Simple alias to make it simpler to pass partial queries around.
type Query<'a, T> = QueryAs<'a, Sqlite, T, sqlx::sqlite::SqliteArguments<'a>>;
