use sqlx::{query::QueryAs, Sqlite};

pub(crate) mod entries;
pub(crate) mod meds;

/// Simple alias to make it simpler to pass partial queries around.
type Query<'a, T> = QueryAs<'a, Sqlite, T, sqlx::sqlite::SqliteArguments<'a>>;

#[macro_export]
macro_rules! generate_functions {
    ($($name:ident<$type:ty>),+) => {
        use futures::StreamExt;
        $(
            /// Generates a vector of results from the given query and database pool.
            /// If results are found, simply return those results.
            /// Otherwise serve empty json.
            async fn $name(query: crate::routes::Query<'_, $type>, limit: usize, db_pool: &SqlitePool) -> Vec<$type> {
                query
                    .fetch(db_pool)
                    .filter_map(|entry| async {
                        match entry {
                            Ok(entry) => Some(entry),
                            Err(_) => None,
                        }
                    })
                    .take(limit)
                    .collect()
                    .await
            }
        )+
    };
}
