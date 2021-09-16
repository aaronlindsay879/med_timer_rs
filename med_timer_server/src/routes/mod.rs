use paperclip::actix::Apiv2Schema;
use serde::Deserialize;
use sqlx::{query::QueryAs, Sqlite};

pub(crate) mod entries;
pub(crate) mod meds;

#[derive(Apiv2Schema, Deserialize)]
struct DefaultQuery {
    count: Option<usize>,
}

impl DefaultQuery {
    pub fn count_or_default(&self) -> usize {
        self.count_or(100)
    }

    pub fn count_or(&self, default: usize) -> usize {
        self.count.unwrap_or(default)
    }
}

/// Simple alias to make it simpler to pass partial queries around.
type Query<'a, T> = QueryAs<'a, Sqlite, T, sqlx::sqlite::SqliteArguments<'a>>;

/// Generates functions which fetch a response from the database, returning a specific type.
///
/// This _should_ be done with generics instead, but issues with resolving lifetimes made that infeasible.
#[macro_export]
macro_rules! generate_response_functions {
    ($($name:ident<$type:ty>),+) => {
        use futures::StreamExt;
        $(
            /// Generates a vector of results from the given query and database pool.
            /// If results are found, simply return those results.
            /// Otherwise serve empty json.
            async fn $name(query: crate::routes::Query<'_, $type>, limit: usize, db_pool: &sqlx::SqlitePool) -> Vec<$type> {
                query
                    .fetch(db_pool)
                    .filter_map(async move |entry| entry.ok())
                    .take(limit)
                    .collect()
                    .await
            }
        )+
    };
}
